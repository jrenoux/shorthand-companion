
use egui::{Align2, Context, Vec2, vec2};

pub struct AddNewWordWindow {
    size: Vec2,
    resizable: bool,
    collapsible: bool,
    anchor: (Align2, Vec2),
    name: String,
    open: bool,
    longhand: String
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
            longhand: "".to_string()
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
        let add_window = egui::Window::new("Add a new word").
            open(&mut self.open)
            .default_size(vec2(512.0, 512.0))
            .collapsible(false)
            .resizable(false);

        add_window.show(ctx, |ui| {
            ui.label("Longhand");
            ui.add(egui::TextEdit::singleline(&mut self.longhand).hint_text("Word in longhand"));
            ui.label(("Shorthand"));

        });
    }

    pub fn activate(&mut self) {
        self.open = true
    }
}

