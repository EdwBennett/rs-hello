# rs_hello

A template for a Rust workspace with a shared library, a CLI, and a
[Zola](https://www.getzola.org/) static site deployed to GitHub Pages.

## Layout

| Path | What it is |
| --- | --- |
| `crates/hello-core` | Shared logic (`todays_primes_message`), compiled both natively (`rlib`, for `hello-cli`) and to WebAssembly (`cdylib`, for the Zola site). |
| `crates/hello-cli` | Command-line consumer of `hello-core`. |
| `site/` | Zola static site — not a Rust crate, no `Cargo.toml` — deployed to GitHub Pages. Loads `hello-core` as WASM to render today's primes in the browser. |
| `.github/workflows` | `ci.yml` (fmt, clippy, tests, WASM + site build check) and `pages.yml` (builds the WASM bundle, then deploys the site). |

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
