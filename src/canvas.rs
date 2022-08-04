use egui::{Frame, Ui, vec2};
use crate::painting::Painting;

pub struct CanvasWindow {
    painting: Painting,
    open: bool
}

impl Default for CanvasWindow {
    fn default() -> Self {
        CanvasWindow {
            painting: Painting::default(),
            open: false,
        }
    }
}

impl CanvasWindow {
    pub fn new() -> Self {
        CanvasWindow {
            painting: Painting::default(),
            open: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return
        }

        // a new window
        let canvas_window = egui::Window::new("Draw your shorthand")
            .default_size(vec2(512.0, 512.0))
            .collapsible(false)
            .resizable(false);

        canvas_window.show(ctx, |ui| {
            //we add the canvas in
            self.display_canvas(ui);
            // we add save and cancel buttons
            ui.horizontal(|ui| {
                if(ui.button("Save").clicked()) {
                    // TODO save
                }
                if(ui.button("Cancel").clicked()) {
                    self.reset();
                    self.open = false;
                }
            });
        });
    }

    pub fn activate(&mut self) {
        self.open = true
    }

    fn display_canvas(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.painting.ui_control(ui);
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.painting.ui_content(ui);
            });
        });
    }

    pub fn reset(&mut self) {
        self.painting.clear_painting();
    }
}

