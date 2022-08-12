use eframe::egui::{self, Ui, RichText};
use crate::student::{Class, Student};

use super::ChooseApp;

pub fn debug_mode(
    choose_app: &mut ChooseApp, 
    ctx: &egui::Context, 
    _frame: &mut eframe::Frame
) {
    egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                show_class_info(&mut choose_app.algorithm.classes[0], ui);
                show_class_info(&mut choose_app.algorithm.classes[1], ui);
                show_class_info(&mut choose_app.algorithm.classes[2], ui);
                show_class_info(&mut choose_app.algorithm.classes[3], ui);
            });
        }
    );
}

fn show_class_info(class: &mut Class, ui: &mut Ui) {
    ui.label(RichText::new(format!("Class {}\nAmount: {}\nCommuters: {}\nNon Commuters: {}", class.get_class_num(), class.get_students().len(), class.get_commuters().len(), class.get_non_commuters().len())).size(20.));
    //ui.label(RichText::new(format!("Amount: {}", class.get_amount())).size(20.));
    //ui.label(RichText::new(format!("Commuters: {}", class.get_commuting_amount())).size(20.));
    //ui.label(RichText::new(format!("Non Commuters: {}", class.get_non_commuting_amount())).size(20.));            
}