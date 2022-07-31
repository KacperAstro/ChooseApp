use eframe::{egui::{self, RichText, Ui}, epaint::Color32};
use super::{ChooseApp, Mode};

struct ChangeLog{
    text: String,
}

impl ChangeLog{
    fn new(text: String) -> Self{
        Self { text }
    }

    fn show(self, ui: &mut Ui) {
        ui.separator();
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(&self.text));
        });
        ui.separator();
    }
}

pub fn start_mode(
    choose_app: &mut ChooseApp, 
    ctx: &egui::Context, 
    _frame: &mut eframe::Frame
) {
    let change_logs: Vec<ChangeLog> = vec![
        ChangeLog::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean commodo luctus justo, a consectetur urna. Integer viverra, nulla a ornare finibus, dui velit mollis tortor, sit amet suscipit leo tortor convallis mi. Praesent et blandit mi, viverra dignissim est. Sed nulla nibh, congue ac rutrum maximus, viverra non nisi. Sed eleifend urna arcu. Phasellus elementum ullamcorper vestibulum. Vestibulum ut dictum magna. Phasellus quis facilisis metus.

        Curabitur sit amet nisi ultricies metus congue pretium. Nulla accumsan sapien eget pellentesque sollicitudin. Praesent mollis ante id enim pharetra pulvinar. Aliquam pulvinar condimentum purus et fermentum. Maecenas ut maximus dolor, sit amet ultrices lectus.".to_string()),
        ChangeLog::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean commodo luctus justo, a consectetur urna. Integer viverra, nulla a ornare finibus, dui velit mollis tortor, sit amet suscipit leo tortor convallis mi. Praesent et blandit mi, viverra dignissim est. Sed nulla nibh, congue ac rutrum maximus, viverra non nisi. Sed eleifend urna arcu. Phasellus elementum ullamcorper vestibulum. Vestibulum ut dictum magna. Phasellus quis facilisis metus.

        Curabitur sit amet nisi ultricies metus congue pretium. Nulla accumsan sapien eget pellentesque sollicitudin. Praesent mollis ante id enim pharetra pulvinar. Aliquam pulvinar condimentum purus et fermentum. Maecenas ut maximus dolor, sit amet ultrices lectus.".to_string()),
        ChangeLog::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean commodo luctus justo, a consectetur urna. Integer viverra, nulla a ornare finibus, dui velit mollis tortor, sit amet suscipit leo tortor convallis mi. Praesent et blandit mi, viverra dignissim est. Sed nulla nibh, congue ac rutrum maximus, viverra non nisi. Sed eleifend urna arcu. Phasellus elementum ullamcorper vestibulum. Vestibulum ut dictum magna. Phasellus quis facilisis metus.

        Curabitur sit amet nisi ultricies metus congue pretium. Nulla accumsan sapien eget pellentesque sollicitudin. Praesent mollis ante id enim pharetra pulvinar. Aliquam pulvinar condimentum purus et fermentum. Maecenas ut maximus dolor, sit amet ultrices lectus.".to_string()),
        ChangeLog::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean commodo luctus justo, a consectetur urna. Integer viverra, nulla a ornare finibus, dui velit mollis tortor, sit amet suscipit leo tortor convallis mi. Praesent et blandit mi, viverra dignissim est. Sed nulla nibh, congue ac rutrum maximus, viverra non nisi. Sed eleifend urna arcu. Phasellus elementum ullamcorper vestibulum. Vestibulum ut dictum magna. Phasellus quis facilisis metus.

        Curabitur sit amet nisi ultricies metus congue pretium. Nulla accumsan sapien eget pellentesque sollicitudin. Praesent mollis ante id enim pharetra pulvinar. Aliquam pulvinar condimentum purus et fermentum. Maecenas ut maximus dolor, sit amet ultrices lectus.".to_string()),
        ChangeLog::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean commodo luctus justo, a consectetur urna. Integer viverra, nulla a ornare finibus, dui velit mollis tortor, sit amet suscipit leo tortor convallis mi. Praesent et blandit mi, viverra dignissim est. Sed nulla nibh, congue ac rutrum maximus, viverra non nisi. Sed eleifend urna arcu. Phasellus elementum ullamcorper vestibulum. Vestibulum ut dictum magna. Phasellus quis facilisis metus.

        Curabitur sit amet nisi ultricies metus congue pretium. Nulla accumsan sapien eget pellentesque sollicitudin. Praesent mollis ante id enim pharetra pulvinar. Aliquam pulvinar condimentum purus et fermentum. Maecenas ut maximus dolor, sit amet ultrices lectus.".to_string()),
    ];

    egui::TopBottomPanel::top("Header").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Welcome to my App").color(Color32::WHITE).strong());
        });
    });

    egui::TopBottomPanel::bottom("Footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.);
            if ui.button("Go to App").clicked() {
                choose_app.mode = Mode::CHOOSE;
            }
            ui.add_space(2.);
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for change_log in change_logs{
                change_log.show(ui);
            }
        });
    });
}
