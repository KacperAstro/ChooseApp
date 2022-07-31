use eframe::egui::{self, Ui, RichText};
use crate::student::Class;

use super::ChooseApp;

pub fn debug_mode(
    choose_app: &mut ChooseApp, 
    ctx: &egui::Context, 
    _frame: &mut eframe::Frame
) {
    egui::TopBottomPanel::top("Top")
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Test");
            });
        }
    );
    egui::TopBottomPanel::bottom("Bottom")
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Test");
            });
        }
    );
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            egui::Grid::new("New Grid").min_col_width(195.)
                .show(ui, |ui|{
                    show_class_info(&choose_app.algorithm.classes[0], ui);
                    show_class_info(&choose_app.algorithm.classes[1], ui);
                    show_class_info(&choose_app.algorithm.classes[2], ui);
                    show_class_info(&choose_app.algorithm.classes[3], ui);
                    ui.end_row();
                }
            );
        }
    );
}

fn show_class_info(class: &Class, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(format!("Class {}", class.get_class_num())).size(20.));
        ui.label(RichText::new(format!("Amount: {}", class.get_amount())).size(20.));
        ui.label(RichText::new(format!("Commuters: {}", class.get_commuting_amount())).size(20.));
        ui.label(RichText::new(format!("Non Commuters: {}", class.get_non_commuting_amount())).size(20.));
    });
}