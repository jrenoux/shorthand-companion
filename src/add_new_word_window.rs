
use egui::{Align2, Context, Frame, Vec2, vec2};
use crate::painting::Painting;

pub struct AddNewWordWindow {
    size: Vec2,
    resizable: bool,
    collapsible: bool,
    anchor: (Align2, Vec2),
    name: String,
    open: bool,
    longhand: String,
    shorthand: Painting,
    missing_longhand: bool,
}

impl Default for AddNewWordWindow {
    fn default() -> Self {
        AddNewWordWindow {
            size: vec2(512.0,512.0),
            resizable: false,
            collapsible: false,
            anchor: (Align2::CENTER_CENTER, vec2(0.0, 0.0)),
            name: "Add new word".to_string(),
            open: false,
            longhand: "".to_string(),
            shorthand: Painting::default(),
            missing_longhand: false
        }
    }
}

impl AddNewWordWindow {
    pub fn new() -> Self {
        AddNewWordWindow::default()
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return
        }
        //show a new window to add a new word
        let add_window = egui::Window::new("Add a new word")
            .default_size(vec2(512.0, 512.0))
            .collapsible(false)
            .resizable(false);

        add_window.show(ctx, |ui| {
            ui.label("Longhand");
            let te_longhand = egui::TextEdit::singleline(&mut self.longhand).hint_text("Word in longhand");
            ui.add(te_longhand);
            if self.missing_longhand {
                ui.label("Please enter a longhand");
            }
            else {
                ui.label("");
            }

            ui.label(("Shorthand"));
            //the painting canvas
            ui.vertical(|ui| {
                self.shorthand.ui_control(ui);
                Frame::canvas(ui.style()).show(ui, |ui| {
                    self.shorthand.ui_content(ui);
                });
            });

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    // check that the text field is not empty
                    if self.longhand.is_empty() {
                        self.missing_longhand = true;
                    }
                    else {
                        self.missing_longhand = false;
                        // TODO save the drawing as picture

                        self.reset_window();
                        self.open = false;
                    }
                };
                if ui.button("Cancel").clicked() {
                    //reset the window
                    self.reset_window();
                    self.open = false;
                };
            });

        });
    }

    fn reset_window(&mut self) {
        self.shorthand.clear_painting();
        self.longhand = "".to_string();
        self.missing_longhand = false;
    }

    pub fn activate(&mut self) {
        self.open = true
    }
}

