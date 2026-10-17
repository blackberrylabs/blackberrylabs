#!/usr/bin/env bash
# Blackberrylabs — descriptions, topics and homepages of the family's repositories, in one
# pass. Idempotent: run it again after a change here. Needs `gh` logged in as the owner.
# Topics are REPLACED (PUT), not merged, so this file is the single source of truth.
set -euo pipefail

ORG=blackberrylabs
TOWER="https://github.com/$ORG/$ORG"
FAMILY="Read-only tools that show what AI coding agents do to your machine — and one screen that gathers them."

set_repo() { # name, homepage, description, topics...
  local name=$1 home=$2 desc=$3; shift 3
  gh repo edit "$ORG/$name" --description "$desc" --homepage "$home" >/dev/null
  local json; json=$(printf '%s\n' "$@" | jq -R . | jq -cs '{names: .}')
  gh api --method PUT "/repos/$ORG/$name/topics" --input - <<<"$json" >/dev/null
  echo "ok  $name  ($# topics)"
}

set_repo blackberrylabs "https://github.com/$ORG" \
  "One screen for everything your AI coding agents do to your machine — permissions, MCP servers, instruction files, disk left behind, AI spend — sorted by severity. Read-only tools, gathered. Linux, Rust." \
  claude-code mcp ai-agents ai-coding ai-tools vibe-coding permissions disk-usage developer-tools audit read-only rust linux blackberrylabs

set_repo permitt "$TOWER" \
  "What Claude Code, Codex, Cursor and Copilot may do on your machine without asking — shell commands, file edits, network, MCP tools — and which setting says so. Read-only. Linux, Rust." \
  claude-code codex cursor copilot mcp permissions allowlist ai-agents ai-security ai-coding ai-tools vibe-coding read-only rust linux blackberrylabs

set_repo mcproster "$TOWER" \
  "Every MCP server declared on your machine in one list — duplicates, dead commands, tokens in clear — across Claude Code, Claude Desktop, Cursor, VS Code and Codex. Read-only. Linux, Rust." \
  mcp mcp-server model-context-protocol claude-code claude-desktop cursor vscode codex inventory ai-agents ai-tools vibe-coding read-only rust linux blackberrylabs

set_repo cleanator "$TOWER" \
  "What AI tooling takes on your disk — Ollama models, Hugging Face and pip caches, Docker images, agent histories — and what each cleanup command would do. Never deletes anything. Linux, Rust." \
  claude-code ollama huggingface docker disk-usage disk-cleaner cleanup treemap ai-agents ai-tools vibe-coding developer-tools rust linux blackberrylabs

set_repo linstruct "$TOWER" \
  "Lint your AI instruction files the way each agent reads them — CLAUDE.md, AGENTS.md, .cursor/rules, GEMINI.md — and see what each host truncates, ignores or shadows. 19 hosts, read-only, every rule sourced." \
  claude-code claude-md agentsmd cursor-rules instruction-files context-engineering linter ai-agents ai-coding ai-tools vibe-coding developer-tools rust linux blackberrylabs

set_repo billfold "$TOWER" \
  "The API keys and accounts your machine holds — Claude Code, Codex, Cursor, Gemini CLI, OpenCode — and the tokens each host burned, priced at API rates. Read-only, no network, no balance fetched. Linux, Rust." \
  api-keys token-usage cost-tracking llm-costs claude-code codex cursor gemini-cli opencode anthropic openai ai-agents ai-tools vibe-coding read-only rust linux blackberrylabs

set_repo bbl-hostrules "$TOWER" \
  "Where each AI coding host keeps its files, and how to read them: sourced host sheets, config readers, secret masking — the foundation under the Blackberrylabs tools. Rust crate." \
  claude-code cursor codex configuration dotfiles xdg mcp ai-tools read-only rust linux blackberrylabs

set_repo bbl-ui "$TOWER" \
  "The shared iced interface of the Blackberrylabs tools: palette, widgets, settings, EN/FR strings. Rust crate." \
  iced rust desktop ui linux-desktop blackberrylabs

set_repo bbl-report "$TOWER" \
  "bbl-report/1, the contract between the Blackberrylabs tower and its modules: report types, validation, four severities. Rust crate." \
  rust schema json cli blackberrylabs

set_repo template "$TOWER" \
  "Starting point for a new Blackberrylabs module: core, CLI and app crates, tested promises, a conforming report, rename.sh. Rust." \
  rust template cli blackberrylabs

# the organisation itself and its profile repository (needs the admin:org scope for the org)
gh repo edit "$ORG/.github" --description "$FAMILY" --homepage "$TOWER" >/dev/null && echo "ok  .github"
gh api --method PATCH "/orgs/$ORG" -f description="$FAMILY" -f blog="$TOWER" >/dev/null \
  && echo "ok  organisation" \
  || echo "!!  organisation: gh lacks the admin:org scope — run: gh auth refresh -s admin:org, then this script again"
