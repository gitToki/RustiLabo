use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "RustiLabo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    cleared: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            cleared: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.cleared {
                return;
            }

            // Centrer "RustiLabo" horizontalement en haut
            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("RustiLabo")
                        .heading()
                        .color(egui::Color32::LIGHT_YELLOW)
                        .size(60.0));
                });
            });

            ui.label(egui::RichText::new("All security vulnerabilities covered : ")
                .color(egui::Color32::WHITE)
                .size(20.0));

            let button_size = egui::vec2(ctx.screen_rect().width() * 0.4, ctx.screen_rect().height() * 0.1);
            if ui.add_sized(button_size, egui::Button::new("Reentrancy Attack")).clicked() {
                self.cleared = true;
            }
        });
    }
}



