use eframe::egui;
use super::{ChooseApp, Mode};

pub fn choose_mode(
    choose_app: &mut ChooseApp,
    ctx: &egui::Context, 
    _frame: &mut eframe::Frame
) {
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            if ui.button("Debug").clicked() {
                choose_app.mode = Mode::DEBUG;
            }
        }
    );
}