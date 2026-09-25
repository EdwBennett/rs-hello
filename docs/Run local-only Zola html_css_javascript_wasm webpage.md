A Zola/Rust site can run entirely on a Fedora KDE machine. Serve the generated site over **localhost HTTP** directly with `file://`: WebAssembly loaders, `fetch`, browser module rules, and routing often expect HTTP and correct MIME types.

For a local-only deployment, bind the server to `127.0.0.1`, which accepts connections only from the same computer—not other devices on your LAN. Zola’s own server does this by default, using the site routinely as a local app.

The practical setup is: build the site once into a fixed directory, then use a user-level `systemd` service to serve that directory at `http://127.0.0.1:8080`.

## **local Zola website summary**

From the root of the Zola project, then run zola serve, then open [http://127.0.0.1:1111](http://127.0.0.1:1111). Zola watches its site inputs and rebuilds while it runs. Its default bind address is `127.0.0.1`, so other machines cannot reach it unless you deliberately change the interface to `0.0.0.0`.

If your Rust/Wasm build has a separate command, run it first—or arrange for its output to land in Zola’s `static/` directory before running Zola:

\# Example only—use your project’s actual Wasm build command  
wasm-pack build \--target web \--out-dir static/pkg  
zola serve

Files below Zola’s `static/` directory are copied directly into the generated site output, so it is a suitable location for the compiled `.wasm`, JS loader, images, and similar prebuilt assets.

## **Persistent local installation**

### **Build into a non-temporary directory**

Do **not** use Zola’s default `public/` directory as your permanent installed copy if you regularly use `zola serve`, since `serve` recreates its output directory. Zola explicitly notes that its output is cleaned/recreated

For example, create a dedicated local web-root:

mkdir \-p "\$HOME/.local/share/my-zola-site"

Build the Wasm bundle using the command your project already uses, then build Zola:

cd /path/to/your/zola-rust-project

\# Replace this with your actual Rust/Wasm build process, if needed.  
\# wasm-pack build \--target web \--out-dir static/pkg

zola build \\  
  \--output-dir "\$HOME/.local/share/my-zola-site" \\  
  \--force

Zola’s `build` command produces the whole static site, and `--output-dir` lets you select a location other than `public/`.

Check the output contains the expected assets:

find "\$HOME/.local/share/my-zola-site" \-maxdepth 2 \-type f | head \-30

You should see `index.html`, CSS, JavaScript, and the `.wasm` bundle or bundles.

### **Test it locally**

Serve the built directory with Python:

cd "\$HOME/.local/share/my-zola-site"  
python3 \-m http.server 8080 \--bind 127.0.0.1

Open:

http://127.0.0.1:8080

The `--bind 127.0.0.1` part matters: it ensures the server is localhost-only. A process listening only on `127.0.0.1` cannot be made network-accessible merely by changing firewall rules.

For modern Python, `.wasm` is normally served as `application/wasm`, which is needed by `WebAssembly.instantiateStreaming`. You can verify the header with:

curl \-I http://127.0.0.1:8080/path/to/your-module.wasm

Look for:

Content-Type: application/wasm

Correct WebAssembly MIME handling is important; an incorrect type can make streaming Wasm compilation fail.

Stop the test server with `Ctrl+C`.

## **Run it automatically with systemd**

Create this service file:

mkdir \-p "\$HOME/.config/systemd/user"

cat \> "\$HOME/.config/systemd/user/my-zola-site.service" \<\<'EOF'  
\[Unit\]  
Description=Local Zola/Rust website  
After=network.target

\[Service\]  
Type=simple  
WorkingDirectory=%h/.local/share/my-zola-site  
ExecStart=/usr/bin/python3 \-m http.server 8080 \--bind 127.0.0.1  
Restart=on-failure  
RestartSec=2

\[Install\]  
WantedBy=default.target  
EOF

Then reload systemd and start it:

systemctl \--user daemon-reload  
systemctl \--user enable \--now my-zola-site.service

Confirm it is active:

systemctl \--user status my-zola-site.service  
curl \-I http://127.0.0.1:8080

Now use this local address in Firefox, Chromium, or another browser:

http://127.0.0.1:8080

To stop it later:

systemctl \--user disable \--now my-zola-site.service

A user service normally ends when the user fully logs out. If you want the website to be running from boot and to remain available after logout, enable systemd “lingering”:

loginctl enable-linger "\$USER"

Lingering keeps the user manager available after logout and can start user services at boot.

## **KDE launcher shortcut**

To make the website feel like a local application, create a KDE application launcher:

mkdir \-p "\$HOME/.local/share/applications"

cat \> "\$HOME/.local/share/applications/my-zola-site.desktop" \<\<'EOF'  
\[Desktop Entry\]  
Type=Application  
Name=My Local Website  
Comment=Open my local Zola/Rust website  
Exec=xdg-open http://127.0.0.1:8080  
Icon=internet-web-browser  
Terminal=false  
Categories=Network;WebBrowser;  
EOF

It should then appear in KDE’s application launcher/search.

## **Important pitfalls**

* **Do not open the site with `file:///…/index.html`.** Use `http://127.0.0.1:8080`. This avoids common module, fetch, and WebAssembly-loading failures.  
* **Keep `base_url` appropriate.** For a local build, configure Zola with a localhost base URL such as `http://127.0.0.1:8080`, particularly if templates generate absolute URLs. Zola requires an HTTP-style `base_url`, not a filesystem path.  
* **Use relative asset URLs where possible.** Zola helpers such as `get_url()` help generate URLs based on the configured site root.  
* **Do not bind to `0.0.0.0`.** That would expose the server to your local network. For local-only use, retain `127.0.0.1`.  
* **Wasm threads may need extra headers.** If your Rust/Wasm app uses `SharedArrayBuffer`, Rayon, threaded Wasm, or similar features, it may require cross-origin isolation headers (`Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy`). A basic Python server does not add those; use Caddy, nginx, or a purpose-built local server in that case. Correct Wasm MIME type alone is not sufficient for threaded Wasm.

For an ordinary Zola \+ Rust/Wasm static site, the `zola build` → Python localhost server → user-level systemd service route is simple, private, and robust on Fedora KDE.

