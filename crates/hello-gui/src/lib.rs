//! The egui app, shared by the native binary and the WebAssembly build.

mod app;

pub use app::HelloApp;

/// Browser entry point: runs on module load and draws into `#the_canvas_id`
/// (see `index.html`).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_web() {
    use wasm_bindgen::JsCast as _;

    let canvas = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("the_canvas_id"))
        .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .expect("canvas element not found");

    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(HelloApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
