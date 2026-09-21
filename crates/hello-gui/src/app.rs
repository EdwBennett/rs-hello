const REPO_URL: &str = "https://github.com/EdwBennett/rs-hello";

pub struct HelloApp {
    name: String,
}

impl HelloApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl eframe::App for HelloApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("hello-gui");
        ui.horizontal(|ui| {
            ui.label("Your name:");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.label(hello_core::greet(&self.name));

        ui.separator();
        ui.hyperlink_to("Source on GitHub", REPO_URL);
        ui.hyperlink_to("Use this template", format!("{REPO_URL}/generate"));
    }
}
