use eframe::epi::{App, Frame};
use egui::{CentralPanel, Context};

impl App for TeelineDict {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        // where the magic happens
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Test test test");
        });
    }

    fn name(&self) -> &str {
        "Teeline Dictionary"
    }
}