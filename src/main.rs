#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, run_native, egui, emath::Vec2};
use gui_part::ChooseApp;

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
      Box::new(ChooseApp::setup(cc))
    }));
}