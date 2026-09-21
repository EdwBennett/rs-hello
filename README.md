# rs_hello

A template for a Rust project whose main product is an [egui](https://github.com/emilk/egui)
app that runs natively **and** in the browser (WebAssembly, hosted on GitHub Pages).

## Layout

| Path | What it is |
| --- | --- |
| `crates/hello-core` | Shared logic (`greet`). Must stay wasm-safe: no filesystem, threads or `Instant`. |
| `crates/hello-gui` | egui app. `lib.rs`/`app.rs` hold the UI; `main.rs` is the native entry, `start_web` (in `lib.rs`) the browser entry. |
| `crates/hello-cli` | Command-line consumer of `hello-core`. |
| `.github/workflows` | `ci.yml` (fmt, clippy, tests, wasm check) and `pages.yml` (deploys the web app). |

## Run

```sh
cargo run -p hello-gui          # native window
cargo run -p hello-cli -- --name Ada
cargo test --workspace
```

Web (needs [Trunk](https://trunkrs.dev): `cargo install trunk`):

```sh
trunk serve --config crates/hello-gui/Trunk.toml
```

On Linux the native GUI needs the usual windowing/GL libraries (Wayland or X11, plus OpenGL).

## Deploying to GitHub Pages

1. In the repo settings, go to **Pages** and set **Source** to **GitHub Actions**.
2. Push to `main`/`master`. `pages.yml` builds with the right `/<repo-name>/` prefix automatically.

## Using this template

1. Click **Use this template** on GitHub (enable *Template repository* in the repo settings first).
2. Rename: replace `hello` in crate names and directories, and update `repository` in the root
   `Cargo.toml` and `REPO_URL` in `crates/hello-gui/src/app.rs`.

## Versions

egui/eframe are pre-1.0 and change APIs between minor releases. They are pinned to a minor
version in the root `Cargo.toml`, and `Cargo.lock` is committed. Upgrade deliberately.
