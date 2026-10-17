//! Assemble: the modules with their installation and report status, and one list of
//! findings across modules, most serious first. The tower sorts; it never reinterprets.

use std::path::PathBuf;
use std::time::Duration;

use bbl_report::{Finding, Report, Severity, Summary};
use serde::Serialize;

use crate::cache::Cache;
use crate::discover::which;
use crate::run::{self, Run, RunError};
use crate::spec::{Modules, Spec};

/// Where a module's report comes from, or why there is none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum ReportState {
    /// Obtained in this run.
    Fresh { elapsed_ms: u64 },
    /// Read from the saved reports.
    Saved { age_s: u64 },
    /// Installed, never run yet.
    NoReport,
    /// Not installed: no binary on `PATH`.
    NotInstalled,
    /// The run or the saved file failed; the reason.
    Failed { error: String },
}

/// One module, as the tower sees it.
#[derive(Debug, Clone, Serialize)]
pub struct ModuleStatus {
    pub spec: Spec,
    pub cli: Option<PathBuf>,
    pub app: Option<PathBuf>,
    pub installed: bool,
    pub state: ReportState,
    pub report: Option<Report>,
}

/// One finding in the common list, with the module it comes from.
#[derive(Debug, Clone, Serialize)]
pub struct Line {
    pub module: String,
    pub module_name: String,
    pub finding: Finding,
}

/// The whole picture.
#[derive(Debug, Clone, Serialize)]
pub struct Dashboard {
    pub modules: Vec<ModuleStatus>,
    /// Every finding of every report, most serious first; ties by family order, then title.
    pub lines: Vec<Line>,
}

impl Dashboard {
    /// Installed modules and their saved reports — nothing is launched.
    pub fn from_saved(modules: &Modules, cache: &Cache, path_dirs: &[PathBuf]) -> Dashboard {
        let statuses = modules
            .specs
            .iter()
            .map(|spec| {
                let cli = which(&spec.cli, path_dirs);
                let app = spec.app.as_deref().and_then(|a| which(a, path_dirs));
                let installed = cli.is_some() || app.is_some();
                let (state, report) = match cache.read(&spec.id) {
                    Some(Ok(saved)) => (ReportState::Saved { age_s: saved.age.as_secs() }, Some(saved.report)),
                    Some(Err(e)) => (ReportState::Failed { error: e }, None),
                    None if installed => (ReportState::NoReport, None),
                    None => (ReportState::NotInstalled, None),
                };
                ModuleStatus { spec: spec.clone(), cli, app, installed, state, report }
            })
            .collect();
        Dashboard::assemble(statuses)
    }

    /// Run one module now, save its report, and update its status. Not installed → the
    /// status says so; a failure keeps the previous report if there was one.
    pub fn refresh(&mut self, id: &str, cache: &Cache) -> Result<Run, RunError> {
        let Some(m) = self.modules.iter_mut().find(|m| m.spec.id == id) else {
            return Err(RunError::Spawn(format!("unknown module {id:?}")));
        };
        let Some(cli) = m.cli.clone() else {
            m.state = ReportState::NotInstalled;
            return Err(RunError::Spawn(format!("{} is not installed ({} not on PATH)", m.spec.name, m.spec.cli)));
        };
        let result = run::report(&m.spec, &cli);
        match &result {
            Ok(run) => {
                if let Err(e) = cache.write(id, &run.json) {
                    m.state = ReportState::Failed { error: e };
                } else {
                    m.state = ReportState::Fresh { elapsed_ms: run.elapsed.as_millis() as u64 };
                }
                m.report = Some(run.report.clone());
            }
            Err(e) => m.state = ReportState::Failed { error: e.to_string() },
        }
        let statuses = std::mem::take(&mut self.modules);
        *self = Dashboard::assemble(statuses);
        result
    }

    /// Run every installed module, in family order.
    pub fn refresh_all(&mut self, cache: &Cache) -> Vec<(String, Result<Duration, RunError>)> {
        let ids: Vec<String> = self.modules.iter().filter(|m| m.cli.is_some()).map(|m| m.spec.id.clone()).collect();
        ids.into_iter()
            .map(|id| {
                let r = self.refresh(&id, cache).map(|run| run.elapsed);
                (id, r)
            })
            .collect()
    }

    fn assemble(modules: Vec<ModuleStatus>) -> Dashboard {
        let mut lines: Vec<(usize, Line)> = Vec::new();
        for (order, m) in modules.iter().enumerate() {
            if let Some(r) = &m.report {
                for f in &r.findings {
                    lines.push((order, Line { module: m.spec.id.clone(), module_name: m.spec.name.clone(), finding: f.clone() }));
                }
            }
        }
        lines.sort_by(|(oa, a), (ob, b)| {
            a.finding
                .severity
                .cmp(&b.finding.severity)
                .then(oa.cmp(ob))
                .then_with(|| a.finding.title.cmp(&b.finding.title))
        });
        Dashboard { modules, lines: lines.into_iter().map(|(_, l)| l).collect() }
    }

    /// How many findings at each severity, strongest first.
    pub fn counts(&self) -> [(Severity, usize); 4] {
        Severity::ALL.map(|s| (s, self.lines.iter().filter(|l| l.finding.severity == s).count()))
    }

    /// The figures of every module that has a report, in family order.
    pub fn summaries(&self) -> Vec<(&ModuleStatus, &Summary)> {
        self.modules.iter().flat_map(|m| m.report.iter().flat_map(move |r| r.summary.iter().map(move |s| (m, s)))).collect()
    }

    /// The lines at or above a severity (1 = act now only … 4 = everything).
    pub fn at_least(&self, min: Severity) -> impl Iterator<Item = &Line> {
        self.lines.iter().filter(move |l| l.finding.severity <= min)
    }

    pub fn module(&self, id: &str) -> Option<&ModuleStatus> {
        self.modules.iter().find(|m| m.spec.id == id)
    }

    pub fn to_json(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("a dashboard serialises");
        s.push('\n');
        s
    }
}
