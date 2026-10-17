#!/usr/bin/env bash
# Gather the modules' released binaries for the full package, from their own packages.
#
# For each sheet in modules/*.toml, takes `<id>_<latest>-1_amd64.deb` — from the sibling
# checkout's dist/ when it is there, else from the module's GitHub release — and extracts it
# into packaging/.bundle/root/ (usr/bin, .desktop files, icons). Nothing is built here: the
# full package ships exactly the binaries the modules released. Needs `ar` and `tar`.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUNDLE="$ROOT/packaging/.bundle"
rm -rf "$BUNDLE/root"
mkdir -p "$BUNDLE/root" "$BUNDLE/debs"

field() { grep -m1 "^$2 *= *\"" "$1" | sed 's/^[^"]*"\([^"]*\)".*/\1/'; }

for sheet in "$ROOT"/modules/*.toml; do
  id="$(field "$sheet" id)"; name="$(field "$sheet" name)"; v="$(field "$sheet" latest)"
  if [ -z "$v" ]; then echo "!! $id: no latest version in its sheet" >&2; exit 2; fi
  deb="${id}_${v}-1_amd64.deb"
  # the sibling checkout is named after the module, first letter upper (Permitt, Mcproster…)
  sibling="$ROOT/../$(echo "${id:0:1}" | tr a-z A-Z)${id:1}/dist/$deb"
  if [ -f "$sibling" ]; then
    cp "$sibling" "$BUNDLE/debs/$deb"; from="sibling dist/"
  elif [ -f "$BUNDLE/debs/$deb" ]; then
    from="already fetched"
  else
    url="https://github.com/blackberrylabs/$id/releases/download/v$v/$deb"
    curl -fsSL -o "$BUNDLE/debs/$deb" "$url"; from="GitHub release"
  fi
  tmp="$(mktemp -d)"
  (cd "$tmp" && ar x "$BUNDLE/debs/$deb" && tar -xf data.tar.* -C "$BUNDLE/root")
  rm -rf "$tmp"
  echo "ok  $name $v ($from)"
done
# the modules' README/copyright are theirs, not the full package's
rm -rf "$BUNDLE/root/usr/share/doc"
echo "== $(ls "$BUNDLE/root/usr/bin" | tr '\n' ' ')"
