use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Rustilabo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        MyApp
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label(egui::RichText::new("          Toutes les failles de sécurité disponibles : ")
                .color(egui::Color32::WHITE)
                .size(30.0));
        });
    }
}

