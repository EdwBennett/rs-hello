# Project rs_hello

## Project Overview
- **Purpose**: a template project.

## Commands
- Developer (not claude code) runs git add / commit / push commands
- A PostToolUse hook (`.claude/settings.json`, script at `.claude/hooks/rust-check.sh`) automatically runs `cargo fmt --all --check`, then `cargo clippy --workspace --all-targets -- -D warnings`, then `cargo test --workspace` after Claude Code edits/writes any `.rs`, `Cargo.toml`, or `Cargo.lock` file, stopping at the first failure and reporting it back. If `cargo test` ever becomes slow enough to be disruptive, tell Claude and it will adjust the hook (e.g. drop `cargo test` from the automatic run, or raise/relax the timeout).
- A second PostToolUse hook (script at `.claude/hooks/rebuild-zola-site.sh`) automatically runs `tools/rebuild-my-zola-site` after Claude Code edits/writes any file under `site/` (except its generated `site/static/wasm/`) or `crates/hello-core/` — rebuilding `hello-core` to WASM, rebuilding the Zola site, and restarting the local `my-zola-site.service`. This assumes Ed's personal local-only setup (see `docs/Run local-only Zola html_css_javascript_wasm webpage.md`) and will fail on any other machine — see the warning in `README.md`.

## Coding Style & Conventions
- Follow idiomatic Rust guidelines for Rust code

## Architecture Guidelines
- hello-cli workspace: Rust terminal
- hello-core workspace: a) Rust core code Lib b) Rust core code wasm 
- site: Zola Framework code html / css / javascript / wasm
