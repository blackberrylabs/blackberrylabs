# Changelog

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
