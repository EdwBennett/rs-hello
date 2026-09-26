#!/usr/bin/env bash
# PostToolUse hook (Edit|Write|MultiEdit matcher).
#
# After Claude Code touches a Rust source file or a Cargo manifest/lock file,
# run cargo fmt --check, cargo clippy (-D warnings) and cargo test in
# sequence, stopping at the first failure. On failure, reports the combined
# output back to Claude via a PostToolUse "block" decision so it can fix the
# problem before continuing.
set -u

file_path=$(jq -r '.tool_input.file_path // empty')

case "$file_path" in
  *.rs|*/Cargo.toml|Cargo.toml|*/Cargo.lock|Cargo.lock)
    ;;
  *)
    exit 0
    ;;
esac

output=$( (
  cargo fmt --all --check &&
  cargo clippy --workspace --all-targets -- -D warnings &&
  cargo test --workspace
) 2>&1 )
status=$?

if [ "$status" -ne 0 ]; then
  jq -n --arg reason "$output" '{decision: "block", reason: ("cargo fmt/clippy/test failed:\n\n" + $reason)}'
fi
