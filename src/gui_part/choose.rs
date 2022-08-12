use eframe::egui::{self, Ui};
use crate::student::Student;

use super::{ChooseApp, Mode};

pub fn choose_mode(
    choose_app: &mut ChooseApp,
    ctx: &egui::Context, 
    _frame: &mut eframe::Frame
) {
    egui::SidePanel::right("The Chosen Ones").resizable(false).min_width(300.)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    for student in choose_app.algorithm.classes[0].get_students() {
                        show_student_info(ui, &student);
                    }
                    
                    for student in &choose_app.algorithm.the_chosen_ones {
                        show_student_info(ui, &student);
                    }
                }
            );
        }
    );

    egui::CentralPanel::default()
        .show(ctx, |ui| {
            if ui.button("Choose").clicked() {
                choose_app.algorithm.choose_students(8);
            }

            if ui.button("Debug").clicked() {
                choose_app.mode = Mode::DEBUG;
            }
        }
    );
}

fn show_student_info(ui: &mut Ui, student: &Student) {
    ui.label(format!("name: {}", student.name));
    ui.label(format!("number of duties: {}", student.number_duties));
    ui.label(format!("is commuting: {}", student.is_commuting));
    ui.separator();
}