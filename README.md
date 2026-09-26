# rs_hello

A template for a Rust workspace with a shared library, a CLI, and a
[Zola](https://www.getzola.org/) static site deployed to GitHub Pages.

> **If you cloned this repo:** It carries personal,
> machine-specific automation: `tools/rebuild-my-zola-site` (see
> `docs/Run local-only Zola html_css_javascript_wasm webpage.md`) assumes a systemd user
> service and local tooling paths, `.claude/settings.json`
> runs that script automatically (via Claude Code hooks) whenever `site/` or
> `crates/hello-core/` changes.

## Layout

| Path | What it is |
| --- | --- |
| `crates/hello-core` | Shared logic (`random_consecutive_primes`, bare numbers only), compiled both natively (`rlib`, for `hello-cli`) and to WebAssembly (`cdylib`, for the Zola site). Presentation (sentence text, DOM updates) lives in each consumer, not here. |
| `crates/hello-cli` | Command-line consumer of `hello-core`. |
| `site/` | Zola static site — not a Rust crate, no `Cargo.toml` — deployed to GitHub Pages. Loads `hello-core` as WASM and builds the displayed sentence in its own inline JS. |
| `.github/workflows` | `ci.yml` (fmt, clippy, tests, WASM + site build check) and `pages.yml` (builds the WASM bundle, then deploys the site). |
| `tools/` | Personal local-dev scripts, not part of the published site or CI — see `docs/Run local-only Zola html_css_javascript_wasm webpage.md`. `rebuild-my-zola-site` is also run automatically by a Claude Code hook (`.claude/settings.json` / `.claude/hooks/`) on `site/` or `hello-core` changes. |

## Run

```sh
cargo run -p hello-cli
cargo test --workspace
```

Site (needs [Zola](https://www.getzola.org/documentation/getting-started/installation/) and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) installed locally, plus the
`wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`):

```sh
cd crates/hello-core
wasm-pack build --target web --out-dir ../../site/static/wasm
cd ../../site
zola serve
```

`site/static/wasm/` is generated (gitignored) — rebuild it with the `wasm-pack` command above
whenever `hello-core` changes.

## Deploying to GitHub Pages

1. In the repo settings, go to **Pages** and set **Source** to **GitHub Actions**.
2. Push to `main`/`master`. `pages.yml` builds `site/` with Zola and deploys it.

## Using this template

1. Click **Use this template** on GitHub (enable *Template repository* in the repo settings first).
2. Rename: replace `hello` in crate names and directories, and update `repository` in the root
   `Cargo.toml`.
3. Update `site/config.toml`: `base_url` (must match `https://<user-or-org>.github.io/<repo>`),
   `title`, `description`.

## Versions

The Zola version used in CI and in the Pages deploy is pinned in both `.github/workflows/ci.yml`
and `.github/workflows/pages.yml` (`ZOLA_VERSION`) — keep the two in sync when bumping it.
