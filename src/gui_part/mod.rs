use eframe::{App, egui::{self, RichText, TextStyle, Ui}, epaint::Color32};
use crate::json_part::read_json;
use crate::student::Student;

pub enum Mode{
    START,
    DATA,
}

pub struct ChooseApp{
    mode: Mode,
    test_data: Vec<Vec<Student>>,
    test_bools: Vec<bool>,
}

impl ChooseApp {
    pub fn new(start_phase: Mode, cc: &eframe::CreationContext<'_>) -> Self {
        let mut test_vec: Vec<Vec<Student>> = Vec::new();
        for x in 2..5 {
            test_vec.push(read_json(format!("classes/class{}.json", x)))
        }
        let test_bools: Vec<bool> = vec![false, false, false, false];

        setup_custom_fonts(&&cc.egui_ctx);

        Self{ mode: start_phase, test_data: test_vec, test_bools }
    }
}

impl App for ChooseApp {
    fn update(
        &mut self, 
        ctx: &eframe::egui::Context, 
        frame: &mut eframe::Frame
    ) {
        match self.mode {
            Mode::START => {
                egui::TopBottomPanel::top("Head").show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(
                            RichText::new("Welcome to My App")
                                .size(30.)
                                .color(Color32::WHITE)
                        );
                    });
                });
                
                egui::TopBottomPanel::bottom("foot").show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.);
                        if ui.button(RichText::new("Go To App").size(24.)).clicked() {
                            self.mode = Mode::DATA;
                        }
                        ui.add_space(3.);
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui|{
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(
                            RichText::new("This is an Test App. I've made it to test a couple of things to my first App. I want to test things like different modes like now you are in Start mode.")
                                .size(24.)
                        );
                        ui.label(RichText::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla ac nibh vitae lacus fermentum iaculis eu vel mauris. Aliquam mollis nisi erat, eu finibus eros dictum non. Aenean vitae suscipit ante. Aliquam varius ultricies libero, vitae ultrices ante maximus eget. Etiam consectetur lacinia libero et volutpat. Nunc rhoncus sollicitudin pulvinar. Nulla in quam ornare, fermentum justo ac, malesuada turpis.

                        Vestibulum eget accumsan velit, et luctus erat. Pellentesque quis maximus felis, accumsan egestas nisl. Cras aliquet nec ante a eleifend. Etiam bibendum justo in ipsum lacinia pretium. Morbi arcu nulla, condimentum quis vulputate ut, feugiat id turpis. Nam ac risus ipsum. Pellentesque pretium egestas enim a porta. Donec mollis sodales nisi. Sed bibendum sed ipsum ut dignissim. Suspendisse mauris urna, ultrices eget auctor eget, semper vitae mi.
                        
                        Phasellus nec lacus in neque fermentum tristique. Nunc sagittis justo non odio ullamcorper, sit amet tempus nisi ullamcorper. Curabitur varius quam id dui pulvinar condimentum. Donec varius magna id interdum feugiat. Sed tincidunt accumsan massa eget condimentum. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec non erat ac nibh consectetur dignissim. Phasellus ut nulla libero. Donec quis magna nec turpis mattis iaculis sit amet nec ligula. Nullam sed bibendum tortor. Maecenas velit sapien, lobortis non iaculis non, bibendum sed magna. Nullam lacinia consequat nulla, quis vehicula risus iaculis vel. Pellentesque viverra, erat et tincidunt lobortis, turpis nibh ultricies sapien, nec rutrum libero erat quis lorem.
                        
                        In interdum non nibh vel luctus. In id lorem rutrum ante euismod pulvinar. Quisque quam nibh, consequat blandit velit nec, aliquet fermentum dolor. Ut rutrum iaculis suscipit. Donec non commodo velit, in hendrerit nunc. Maecenas volutpat nec lacus et rhoncus. Duis egestas libero non ante vehicula finibus. Aliquam erat volutpat. Morbi neque purus, dignissim non interdum consequat, luctus vel dolor. Duis sit amet neque non lorem facilisis consequat in at velit. Phasellus sodales interdum justo ac semper. Aenean id pretium erat, ut vestibulum lacus.
                        
                        Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Etiam rhoncus quis diam at consectetur. Duis lorem velit, tincidunt nec nulla non, posuere feugiat nisl. Nullam interdum, libero ut ullamcorper varius, metus metus laoreet ante, vitae scelerisque massa felis dictum velit. Sed pretium mi sed enim mattis vestibulum at vitae dolor. Nam placerat, sapien congue blandit lobortis, ipsum ex dictum risus, id faucibus arcu ipsum ut erat. Aliquam consectetur eros a dui lobortis, ac feugiat urna.")
                                .size(24.)
                        );
                    });
                });
            }
            
            Mode::DATA => {
                egui::CentralPanel::default().show(ctx, |ui| {                    
                    // Class 1
                    showing_class(self, ui, 1);

                    // Class 2
                    showing_class(self, ui, 2);

                    // Class 3
                    showing_class(self, ui, 3);

                    // Class 4
                    //showing_class(self, ui, 4);
                });
                egui::SidePanel::right("right").min_width(300.).resizable(false).show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match self.test_bools[..] {
                            [true, false, false, false] => {
                                render_student(&self.test_data[0], ui);
                            }
                            [false, true, false, false] => {
                                render_student(&self.test_data[1], ui);
                            }
                            [false, false, true, false] => {
                                render_student(&self.test_data[2], ui);
                            }
                            [false, false, false, true] => {
                                ui.label(RichText::new("Sorry I didn't add first class :D").color(Color32::WHITE));
                                //render_student(&self.test_data[3], ui)
                            }
                            _ => {
                                ui.label(RichText::new("Check any class to see something here :)").color(Color32::WHITE));
                            }
                        }
                    });
                });
            }
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut style = (*ctx.style()).clone();

    style.text_styles.get_mut(&TextStyle::Small).unwrap().size = 24.;
    style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 24.;
    style.text_styles.get_mut(&TextStyle::Monospace).unwrap().size = 24.;
    style.text_styles.get_mut(&TextStyle::Button).unwrap().size = 24.;
    style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = 24.;

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../fonts/Roboto-Regular.ttf")),    
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "my_font".to_owned());
    
    ctx.set_fonts(fonts);
    ctx.set_style(style);
}

fn render_student(data: &Vec<Student>, ui: &mut Ui) {
    for test in data.iter() {
        ui.horizontal(|ui| {
            ui.label(RichText::new("name:").color(Color32::WHITE));
            ui.label(RichText::new(format!("{}", test.name)));
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("number of duties:").color(Color32::LIGHT_BLUE));
            ui.label(RichText::new(format!("{}", test.number_duties)));
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Is commuting?:").color(Color32::LIGHT_RED));
            ui.label(RichText::new(format!("{}", test.is_commuting)));
        });
        
        ui.separator();
    }
}

fn showing_class(chooseapp: &mut ChooseApp, ui: &mut Ui, class_num: usize) {
    ui.push_id(class_num, |ui| {
        let collapsing_response = egui::CollapsingHeader::new(format!("Class {}", class_num))
            .show(ui, |ui| {
                ui.label(RichText::new(format!("Amound of People: {}", chooseapp.test_data[class_num - 1].len())));
            }
        );
        ui.separator();
        chooseapp.test_bools[class_num - 1] = !collapsing_response.fully_closed();
    });
    
}

fn count_commuting(class: Vec<Student>) -> u32 {
    todo!()
}