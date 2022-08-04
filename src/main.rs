use std::io::Error;
use eframe::{NativeOptions, run_native};
use crate::model::teeline_dict;
use crate::app::TeelineApp;

mod model;
mod app;
mod add_new_word_window;
mod painting;
mod utils;
mod canvas;


fn main() {
    let options = NativeOptions::default();
    run_native("Shorthand Dictionary", options, Box::new(|cc| Box::new(TeelineApp::new(cc))));
}