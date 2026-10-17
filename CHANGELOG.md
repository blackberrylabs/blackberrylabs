# Changelog

## [Unreleased]

- Desktop app `bbl-app` (iced, `bbl-ui`): the sentence, the four severities, the module
  cards with the modules' own figures, the list and the detail, open-in-module; saved
  reports shown at once and refreshed in the background; English and French; light and
  dark. Packaging: `.deb`, `.rpm`, AppImage.
- Core and command line: module sheets (`modules/*.toml`), discovery on `PATH`, launch with
  a time limit, saved reports under `~/.local/state/blackberrylabs/`, one list of findings
  across modules sorted by severity; `bbl`, `bbl refresh`, `bbl modules`, `--json`,
  `--min-severity`, `--module`.
