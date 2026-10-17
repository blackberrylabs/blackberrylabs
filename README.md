# Blackberrylabs

**Everything your AI agents do to your machine, on one screen. Read-only, like the tools
behind it.**

Blackberrylabs is the control tower of a small family of desktop tools for developers who
run AI on Linux:

| Module | Verb | What it answers |
|---|---|---|
| [Cleanator](https://github.com/blackberrylabs/cleanator) | clean | What AI tooling left on the disk, and what could go |
| [MCProster](https://github.com/blackberrylabs/mcproster) | inventory | Every MCP server declared on the machine, once, with what is wrong |
| [Permitt](https://github.com/blackberrylabs/permitt) | inspect | What each AI assistant may do without asking, and which setting says so |

The tower computes nothing. It launches the installed modules, reads their reports
([`bbl-report/1`](https://github.com/blackberrylabs/bbl-report)), merges them into one list
of findings sorted by severity — act now, review, good to know, information — and opens the
module at the right place. A module that is not installed is shown with where to get it.

## Command line

```sh
bbl                    # the findings from the saved reports, most serious first
bbl refresh            # run the installed modules, save their reports, show the list
bbl refresh permitt    # one module
bbl modules            # the family: installed or not, age of each report
bbl --min-severity 2   # act now and review only
bbl --module permitt   # one module's findings
bbl --json             # for scripts
```

`bbl` never launches anything by itself; `bbl refresh` does, and the only command it ever
runs is each module's own `report`. Reports are saved by the tower under
`~/.local/state/blackberrylabs/<module>/report.json` (`$XDG_STATE_HOME` honoured,
`$BBL_STATE_DIR` overrides); nothing else is written.

## Adding a module

A module is a binary that answers `<module> report` with a `bbl-report/1` document, plus
one declarative sheet in `modules/<id>.toml` (name, verb, binaries, report arguments, time
limit, home page). The tower holds no code specific to a module.

## How it is built

A Rust workspace: `crates/core` (module sheets, discovery on `PATH`, launch with a time
limit, saved reports, assembly and sorting), `crates/cli` (`bbl`); the desktop app `bbl-app`
comes next, on [`bbl-ui`](https://github.com/blackberrylabs/bbl-ui). `#![forbid(unsafe_code)]`.
The design brief, plan and journal are kept in French at the root; code, comments and
documentation are in English.

By [Alexandre Do](https://github.com/blackberrylabs).

## License

MIT — see [LICENSE](LICENSE).
