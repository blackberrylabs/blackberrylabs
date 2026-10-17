# Adding a module to Blackberrylabs

A module is a tool of its own — its repository, its releases, its users — that the control
tower can also run and read. The tower holds no code specific to any module: a module plugs
in through the report contract and one declarative sheet. This page is the whole procedure.

## 0. Start from the template

[`blackberrylabs/template`](https://github.com/blackberrylabs/template) is a Rust workspace
that already keeps the family's promises (read-only, no network, secrets masked, actions
shown never run, every rule sourced), already speaks the contract, and already packages as
`.deb`, `.rpm` and AppImage.

```sh
git clone https://github.com/blackberrylabs/template newname && cd newname
./rename.sh newname "Newname" && rm rename.sh
cargo test
```

Then replace `crates/core/src/scan.rs` (the read) and `model.rs` (the module's kinds of
finding). If the module reads the files of AI hosts, add
[`bbl-hostrules`](https://github.com/blackberrylabs/bbl-hostrules) rather than writing
locations, readers or secret masking again.

## 1. `<module> report` — the contract

The command line answers `report` with one `bbl-report/1` document on stdout — the types,
the validation and the conformance check are in
[`bbl-report`](https://github.com/blackberrylabs/bbl-report). What the document holds:

| Field | What goes in |
|---|---|
| `module` | `id` (slug, the same as the sheet), `name`, `version`, `verb` (one word: `clean`, `inventory`, `inspect`…), `tagline`, `app` and `cli` (binary names), `home` |
| `summary` | 1 to 4 figures — the ones the app shows on its cards — each with a `tone` (`neutral`, `good`, `amber`, `alert`) |
| `findings` | One per finding, with a stable `id` (same thing, same id, run after run), a `severity` (below), the module's own `kind` slug, a `title`, a `subject`, a `detail`, `evidence` (where it was seen), `actions` (shown, never run: `hint`, `open-file`, `open-url`, `command`), and `open` (how to open the app there) |
| `notes`, `errors` | Short sentences: what was read, what could not be |

**Nothing new in the report**: only what the CLI and the app already show, secrets masked
upstream. The tower displays; it never reinterprets, never runs an action, never follows a
link by itself.

Pick the grain that keeps the tower's list readable: Permitt and MCProster send one line per
finding (a handful each); Cleanator sends one line per rule group, not one per file;
Linstruct leaves out what concerns hosts that are not on the machine (its app shows them as
"what if").

The module's tests run `bbl_report::conformance::check` on the module's own output — the
template does.

## 2. The four severities — the mapping is the module's

| Wire | Name | Meaning |
|---|---|---|
| 1 | act now | data loss possible, a secret in clear, a bypass in a shell file |
| 2 | review | acts without asking, broad access, a large recoverable volume |
| 3 | good to know | a duplicate, versions that differ, space that could go |
| 4 | information | disabled, not cached, not readable |

Each kind of finding gets its place in the module's code (`Kind::severity` in the template)
and **the mapping is written in the module's CHANGELOG** — users of the tower read it there.
Existing mappings, for consistency:

- Permitt: shell bypass → 1; runs without asking, a model decides → 2; deny list only, not
  honoured → 3; unreadable, contradictory, unknown → 4.
- MCProster: secret in clear → 1; wide access, confirmations skipped → 2; duplicate,
  versions differ, command missing, context cost → 3; unreadable, disabled, not in cache → 4.
- Cleanator: a group that frees 1 GiB or more → 2; less → 3; `never` → 4.
- Linstruct: a host cuts or skips content, a secret in clear → 1; past a documented limit
  with undocumented consequence, a file the host never reads, a dead `@` reference, a front
  matter that does not apply as written → 2; shadowed, over the recommended length, a
  duplicate, HTML comments sent to the model, a legacy name → 3; empty, unreadable → 4.

## 3. `--select` in the app

`open.app` and `open.args` in each finding say how to open the module there. The app must
accept the argument the report announces — `--select <something the module understands>`:
a cell for Permitt (`host:capability`), a server for MCProster (its identity), a rule group
or an item for Cleanator (`rule-id` or `rule-id::key`), with the scanned path first. The
tower launches `open.app` with `open.args` as they are.

## 4. The sheet in the tower

One file in the tower's `modules/`, embedded at build time:

```toml
id = "newname"                       # the same as module.id in the report
name = "Newname"
verb = "inspect"
tagline = "One sentence, as shown on the module card"
description = "A short paragraph, shown when the module is not installed."
cli = "newname"                      # looked up on PATH
app = "newname-app"
report = ["report"]                  # the arguments that print the report
timeout_s = 60                       # 900 for a module that scans a disk
schema = "bbl-report/1"
home = "https://github.com/blackberrylabs/newname"
releases = "https://github.com/blackberrylabs/newname/releases"
```

The tower's tests (`crates/core/tests/tower.rs`) run on fake modules that print real
reports: add the new module's report to `tests/fixtures/reports/` (paths scrubbed to
`/home/me`) so the tower is tested against it. Then a tower release carries the sheet.

## 5. Before the release

- A real pass on a real machine, for the module and for `bbl refresh` with it installed.
- The module's README says it is part of the family and links to the tower; the tower's
  README and the organisation profile list the module.
- Release the module first (so "Get it" lands on a package that speaks the contract), the
  tower after.

## What the tower does with all this

Launches `<cli> <report args>` with the time limit, checks the output with
`bbl_report::conformance::check`, saves it under
`~/.local/state/blackberrylabs/<id>/report.json`, merges every module's findings into one
list sorted by severity, then family order, then title — and opens `open.app` with
`open.args` when asked. Nothing else.
