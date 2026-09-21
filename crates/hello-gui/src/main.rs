fn main() -> eframe::Result {
    eframe::run_native(
        "hello-gui",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(hello_gui::HelloApp::new(cc)))),
    )
}
