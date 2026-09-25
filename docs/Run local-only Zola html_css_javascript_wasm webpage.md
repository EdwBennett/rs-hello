A Zola/Rust site can run entirely on a Fedora KDE machine. Serve the generated site over **localhost HTTP**, not `file://`: WebAssembly loaders, `fetch`, browser module rules, and routing often expect HTTP and correct MIME types.

For a local-only deployment, bind the server to `127.0.0.1`, which accepts connections only from the same computer—not other devices on your LAN. Zola’s own server does this by default, using the site routinely as a local app.

The practical setup is: build the site once into a fixed directory, then use a user-level `systemd` service to serve that directory at `http://127.0.0.1:8080`.

## **local Zola website summary**

From the root of the Zola project, then run zola serve, then open [http://127.0.0.1:1111](http://127.0.0.1:1111). Zola watches its site inputs and rebuilds while it runs. Its default bind address is `127.0.0.1`, so other machines cannot reach it unless you deliberately change the interface to `0.0.0.0`.

If your Rust/Wasm build has a separate command, run it first—or arrange for its output to land in Zola’s `static/` directory before running Zola:

```sh
# Example only—use your project’s actual Wasm build command
wasm-pack build --target web --out-dir static/pkg
zola serve
```

Files below Zola’s `static/` directory are copied directly into the generated site output, so it is a suitable location for the compiled `.wasm`, JS loader, images, and similar prebuilt assets.

## **Persistent local installation**

### **Build into a non-temporary directory**

Do **not** use Zola’s default `public/` directory as your permanent installed copy if you regularly use `zola serve`, since `serve` recreates its output directory. Zola explicitly notes that its output is cleaned/recreated

For example, create a dedicated local web-root:

```sh
mkdir -p "$HOME/.local/share/my-zola-site"
```

Build the Wasm bundle using the command your project already uses, then build Zola:

```sh
cd /path/to/your/zola-rust-project

# Replace this with your actual Rust/Wasm build process, if needed.
# wasm-pack build --target web --out-dir static/pkg

zola build \
  --base-url http://127.0.0.1:8080 \
  --output-dir "$HOME/.local/share/my-zola-site" \
  --force
```

The `--base-url` flag matters as much as `--output-dir`: unlike `zola serve`, `zola build` does **not** substitute a local address on its own—it uses `base_url` from `config.toml` verbatim. If that config value is a production URL (as it typically is, for a site that's also deployed elsewhere), templates that call `get_url()` will emit absolute links back to the *live* site instead of the local copy sitting next to it—so assets load over the network from production rather than from disk, and the "local-only" build silently depends on internet access and stops working offline. Passing `--base-url http://127.0.0.1:8080` here overrides `config.toml` for this build only, so generated links point at the local server instead.

Zola’s `build` command produces the whole static site, and `--output-dir` lets you select a location other than `public/`.

Check the output contains the expected assets:

```sh
find "$HOME/.local/share/my-zola-site" -maxdepth 2 -type f | head -30
```

You should see `index.html`, CSS, JavaScript, and the `.wasm` bundle or bundles.

### **Test it locally**

Serve the built directory with Python:

```sh
cd "$HOME/.local/share/my-zola-site"
python3 -m http.server 8080 --bind 127.0.0.1
```

Open:

http://127.0.0.1:8080

The `--bind 127.0.0.1` part matters: it ensures the server is localhost-only. A process listening only on `127.0.0.1` cannot be made network-accessible merely by changing firewall rules.

For modern Python, `.wasm` is normally served as `application/wasm`, which is needed by `WebAssembly.instantiateStreaming`. You can verify the header with:

```sh
curl -I http://127.0.0.1:8080/path/to/your-module.wasm
```

Look for:

```
Content-Type: application/wasm
```

Correct WebAssembly MIME handling is important; an incorrect type can make streaming Wasm compilation fail.

Stop the test server with `Ctrl+C`.

## **Run it automatically with systemd**

Create this service file:

```sh
mkdir -p "$HOME/.config/systemd/user"

cat > "$HOME/.config/systemd/user/my-zola-site.service" <<'EOF'
[Unit]
Description=Local Zola/Rust website
After=network.target

[Service]
Type=simple
WorkingDirectory=%h/.local/share/my-zola-site
ExecStart=/usr/bin/python3 -m http.server 8080 --bind 127.0.0.1
Restart=on-failure
RestartSec=2

[Install]
WantedBy=default.target
EOF
```

Then reload systemd and start it:

```sh
systemctl --user daemon-reload
systemctl --user enable --now my-zola-site.service
```

Confirm it is active:

```sh
systemctl --user status my-zola-site.service
curl -I http://127.0.0.1:8080
```

Now use this local address in Firefox, Chromium, or another browser:

http://127.0.0.1:8080

### **Updating the site after a source change**

`zola build` doesn't update files in place — its documented behavior is to delete the whole
output directory and recreate it fresh on every run. If the systemd service is already running
when you rebuild, its process keeps its *original* handle to that directory: the path still
resolves and serving still works, but `/proc/<pid>/cwd` will show `(deleted)`, meaning the
running process is holding an orphaned filesystem reference instead of a clean one pointing at
the directory that now actually exists at that path. Restarting the service after every rebuild
avoids that.

Rather than remembering three separate commands (rebuild the WASM bundle, rebuild the site,
restart the service) in the right order every time the Rust/WASM source changes, they're combined
into one script tracked in this repo: [`tools/rebuild-my-zola-site`](../tools/rebuild-my-zola-site).
It's kept in the repo (rather than only in `~/.local/bin`, where `zola` itself still lives) on
purpose: `~/.local/bin` isn't backed up by cloning this repo, but the repo is — so if this machine
is lost, rebuilt, or replaced, `git clone` alone recovers the script, not just its documentation.

It's a personal local-dev tool, not project tooling — it hardcodes a port and a systemd unit name
that are this machine's choices, not the template's. It self-locates the repo root from its own
path, though, so it isn't tied to `$HOME/src/rs_hello` specifically; it works from wherever the
repo happens to be cloned. `wasm-bindgen` is called directly (rather than `wasm-pack build`)
because that's the toolchain actually installed locally; either works, as long as the `--out-dir`
matches what `index.html`'s `get_url(path='wasm/...')` call expects.

Run it via a symlink from somewhere on `PATH`, e.g.:

```sh
ln -s "$(pwd)/tools/rebuild-my-zola-site" "$HOME/.local/bin/rebuild-my-zola-site"
```

Whenever `crates/hello-core` or `site/` changes and you want the local copy to reflect it, run
`rebuild-my-zola-site`. **On a new machine**, after re-doing the `zola`/`wasm-bindgen`/systemd/KDE
setup above from this doc, the one step that's easy to forget is re-creating this symlink — the
script itself comes back automatically with `git clone`, but the `~/.local/bin` entry pointing at
it does not.

To stop it later:

```sh
systemctl --user disable --now my-zola-site.service
```

A user service normally ends when the user fully logs out. If you want the website to be running from boot and to remain available after logout, enable systemd “lingering”:

```sh
loginctl enable-linger "$USER"
```

Lingering keeps the user manager available after logout and can start user services at boot.

## **KDE launcher shortcut**

To make the website feel like a local application, create a KDE application launcher:

```sh
mkdir -p "$HOME/.local/share/applications"

cat > "$HOME/.local/share/applications/my-zola-site.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=My Local Website
Comment=Open my local Zola/Rust website
Exec=xdg-open http://127.0.0.1:8080
Icon=internet-web-browser
Terminal=false
Categories=Network;WebBrowser;
EOF
```

It should then appear in KDE’s application launcher/search.

## **Important pitfalls**

* **Do not open the site with `file:///…/index.html`.** Use `http://127.0.0.1:8080`. This avoids common module, fetch, and WebAssembly-loading failures.
* **Keep `base_url` appropriate.** For a local build, pass `--base-url http://127.0.0.1:8080` to `zola build` (it does not localhost itself automatically, unlike `zola serve`), particularly if templates generate absolute URLs via `get_url()`. Zola requires an HTTP-style `base_url`, not a filesystem path.
* **Use relative asset URLs where possible.** Zola helpers such as `get_url()` help generate URLs based on the configured site root.
* **Do not bind to `0.0.0.0`.** That would expose the server to your local network. For local-only use, retain `127.0.0.1`.
* **Wasm threads may need extra headers.** If your Rust/Wasm app uses `SharedArrayBuffer`, Rayon, threaded Wasm, or similar features, it may require cross-origin isolation headers (`Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy`). A basic Python server does not add those; use Caddy, nginx, or a purpose-built local server in that case. Correct Wasm MIME type alone is not sufficient for threaded Wasm.

For an ordinary Zola + Rust/Wasm static site, the `zola build` → Python localhost server → user-level systemd service route is simple, private, and robust on Fedora KDE.
