#!/usr/bin/env bash
# PostToolUse hook (Edit|Write|MultiEdit matcher).
#
# After Claude Code touches a file under site/ (the Zola site itself,
# excluding its generated site/static/wasm output) or crates/hello-core/
# (the shared Rust core the site loads as WASM), rebuild and redeploy the
# local dev copy via tools/rebuild-my-zola-site.
#
# That script assumes Ed's personal local-only setup (wasm-bindgen on PATH,
# a my-zola-site.service systemd user unit, etc. — see
# docs/Run local-only Zola html_css_javascript_wasm webpage.md). On a
# machine without that setup it will fail; see the warning in README.md.
set -u

file_path=$(jq -r '.tool_input.file_path // empty')

case "$file_path" in
  */site/static/wasm/*)
    # Generated output of this very script; rebuilding in response to it
    # would just redo the work that produced it.
    exit 0
    ;;
  */site/*|*/crates/hello-core/*)
    ;;
  *)
    exit 0
    ;;
esac

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

output=$("$repo_root/tools/rebuild-my-zola-site" 2>&1)
status=$?

if [ "$status" -ne 0 ]; then
  jq -n --arg reason "$output" '{decision: "block", reason: ("tools/rebuild-my-zola-site failed:\n\n" + $reason)}'
fi
