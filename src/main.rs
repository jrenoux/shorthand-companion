use std::io::Error;
use eframe::{NativeOptions, run_native};
use crate::model::teeline_dict;
use crate::app::TeelineApp;

mod model;
mod app;



fn main() {
    let options = NativeOptions::default();
    run_native("TeeLine Dictionary", options, Box::new(|cc| Box::new(TeelineApp::new(cc))));
}