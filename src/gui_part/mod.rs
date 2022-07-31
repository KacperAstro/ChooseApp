use crate::algorithms::Algorithm;
use eframe::{egui::{self, TextStyle}, App};

use self::{start::start_mode, debug::debug_mode, choose::choose_mode};

mod start;
mod debug;
mod choose;

enum Mode {
    START,
    DEBUG,
    CHOOSE,
}

pub struct ChooseApp {
    mode: Mode,
    algorithm: Algorithm,
}

impl ChooseApp {
    pub fn setup(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let algorithm = Algorithm::new();

        Self {
            mode: Mode::START,
            algorithm
        }
    }
}

impl App for ChooseApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.mode {
            Mode::START => start_mode(self, ctx, frame),

            Mode::DEBUG => debug_mode(self, ctx, frame),

            Mode::CHOOSE => choose_mode(self, ctx, frame),
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
    style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = 30.;

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