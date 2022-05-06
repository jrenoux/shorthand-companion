use eframe::{NativeOptions, run_native};
use crate::app::TeelineDict;

mod model;
mod app;



fn main() {
    let app: TeelineDict = TeelineDict;
    let options = NativeOptions::default();
    run_native(Box::new(app), options);
}