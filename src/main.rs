#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, run_native, egui, emath::Vec2};
use gui_part::{ChooseApp, Mode};

// adding everything together
mod algorithms;
mod json_part;
mod txt_part;
mod student;
mod gui_part;

fn main() {
  let (width, height) = (800, 800);
  let mut options = NativeOptions::default();
  options.initial_window_size = Some(Vec2::new(width as f32, height as f32));
  
  run_native(
    "ChooseApp",
    options, 
    Box::new(|cc|{ 
      cc.egui_ctx.set_visuals(egui::Visuals::dark());
      Box::new(ChooseApp::new(Mode::DATA, cc))
    }));
}

/*
    TODO:
      - Do a mock up of the App
        - Think about the look
        - How the Data part should work

    MAYBE:
      - Change a choosing algorithm to be based on the
        percent of the class to the amount of people

      - Make a choosing algorithm
        - Choosing one commut&ing, one not
        - Choosing two commuting
        - Choosing two not commuting

    DONE:
    [X] Made a start screen of the App
    [X] Write Txt file to Json file
    [X] Reading Json file to vec of student structs
*/