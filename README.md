# rs_hello

A template for a Rust workspace with a shared library, a CLI, and a
[Zola](https://www.getzola.org/) static site deployed to GitHub Pages.

## Layout

| Path | What it is |
| --- | --- |
| `crates/hello-core` | Shared logic (`greet`). |
| `crates/hello-cli` | Command-line consumer of `hello-core`. |
| `site/` | Zola static site — not a Rust crate, no `Cargo.toml` — deployed to GitHub Pages. |
| `.github/workflows` | `ci.yml` (fmt, clippy, tests, site build check) and `pages.yml` (deploys the site). |

## Run

```sh
cargo run -p hello-cli -- --name Ada
cargo test --workspace
```

Site (needs [Zola](https://www.getzola.org/documentation/getting-started/installation/) installed
locally):

```sh
cd site
zola serve
```

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
