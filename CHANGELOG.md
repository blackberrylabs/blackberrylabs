# Changelog

## [0.3.1] — 2026-10-17

- Packaging only: in the full package the modules' menu entries are hidden (`NoDisplay`) —
  one piece of software in the menu, the tower, which opens each module itself. The unit
  packages and the tower alone keep their entries. Binaries unchanged.

## [0.3.0] — 2026-10-17

- Fifth module: **Billfold** 0.1.0 (`tally`) — the accounts the machine is wired to, the
  keys it holds, the tokens each host burned. Sheet `modules/billfold.toml`, fixture report
  in the tests (from Billfold's own fake home, no real machine data).
- A click on a module keeps you in the tower, filtered on it, with its figures and findings
  in the pane; its own app opens only from an explicit *Open in …* button (decision T6:
  "one tool, not a launcher").
- *Get it* opens the exact package for this machine — `.deb` on the Debian family, `.rpm`
  on the RPM family, AppImage elsewhere, from `/etc/os-release` (the system's identity, read
  once, not a host file) — of the latest version each sheet knows (`latest`), so the browser
  downloads it; *All versions* keeps the releases page. The tower still never fetches
  anything (R3).
- **The full package**: `blackberrylabs-full` (`.deb`, `.rpm`, AppImage) ships the tower and
  the five modules' released binaries, gathered from their own packages by
  `packaging/bundle.sh` — one install for someone who wants several tools; the tower alone
  stays for someone who adds modules one by one. The tower looks for modules next to its
  own binary first (an AppImage's `usr/bin` is not on `PATH`), then on `PATH`.
- README: the family sentence, the five modules, one module alone or several with the tower;
  `packaging/github_meta.sh` sets the descriptions, topics and homepages of every repository
  of the family from one file.

## [0.2.0] — 2026-10-17

- Fourth module: **Linstruct** 0.1.0 (`lint`) — your AI instruction files (CLAUDE.md,
  AGENTS.md, .cursor/rules…) as each agent will actually read them, and what it silently
  drops. Sheet `modules/linstruct.toml`, fixture report in the tests, time limit 120 s.
- `docs/new-module.md`: how a module joins the family, with the module template
  (`blackberrylabs/template`); Linstruct's severity mapping added to the examples.

## [0.1.0] — 2026-10-17

First release of the control tower, with three modules that speak the contract: Cleanator
0.2.0, MCProster 0.2.1, Permitt 0.2.0.

- Desktop app `bbl-app` (iced, `bbl-ui`): the sentence, the four severities, the module
  cards with the modules' own figures, the list and the detail, open-in-module; saved
  reports shown at once and refreshed in the background; English and French; light and
  dark. Packaging: `.deb`, `.rpm`, AppImage.
- Core and command line: module sheets (`modules/*.toml`), discovery on `PATH`, launch with
  a time limit, saved reports under `~/.local/state/blackberrylabs/`, one list of findings
  across modules sorted by severity; `bbl`, `bbl refresh`, `bbl modules`, `--json`,
  `--min-severity`, `--module`.
