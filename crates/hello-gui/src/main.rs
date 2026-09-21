#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "hello-gui",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(hello_gui::HelloApp::new(cc)))),
    )
}

// The browser entry point is `start_web` in lib.rs; this only exists so the
// package still builds for wasm32.
#[cfg(target_arch = "wasm32")]
fn main() {}
