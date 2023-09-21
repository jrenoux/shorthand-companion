use std::io::Error;
use eframe::{NativeOptions, run_native};
use crate::model::shorthand_dict;
use crate::app::ShorthandApp;

mod model;
mod app;
mod add_new_word_window;
mod painting;
mod utils;
mod canvas;


fn main() {
    let options = NativeOptions::default();
    run_native("Shorthand Companion", options, Box::new(|cc| Box::new(ShorthandApp::new(cc))));
}