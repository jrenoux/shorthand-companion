use std::fs;
use eframe::epaint::TextureHandle;
use egui::{Align2, Context, Frame, Ui, Vec2, vec2};
use image::ImageError;
use rfd;
use crate::canvas::CanvasWindow;
use crate::painting::Painting;
use crate::utils;

pub struct AddNewWordWindow {
    dict_location: String,
    size: Vec2,
    resizable: bool,
    collapsible: bool,
    anchor: (Align2, Vec2),
    name: String,
    open: bool,
    longhand: String,
    shorthand_painting: CanvasWindow,
    shorthand_path: String,
    error_image_file: String,
    missing_longhand: bool,
    missing_shorthand: bool,
}

impl Default for AddNewWordWindow {
    fn default() -> Self {
        AddNewWordWindow {
            dict_location: "./dictionary/".to_string(),
            size: vec2(512.0,512.0),
            resizable: false,
            collapsible: false,
            anchor: (Align2::CENTER_CENTER, vec2(0.0, 0.0)),
            name: "Add new word".to_string(),
            open: false,
            longhand: "".to_string(),
            shorthand_painting: CanvasWindow::default(),
            shorthand_path: "".to_string(),
            error_image_file: "".to_string(),
            missing_longhand: false,
            missing_shorthand: false,
        }
    }
}

impl AddNewWordWindow {
    pub fn new(location: String) -> Self {
        // TODO see how I can use the default for the rest
        AddNewWordWindow {
            dict_location: location,
            size: vec2(512.0,512.0),
            resizable: false,
            collapsible: false,
            anchor: (Align2::CENTER_CENTER, vec2(0.0, 0.0)),
            name: "Add new word".to_string(),
            open: false,
            longhand: "".to_string(),
            shorthand_painting: CanvasWindow::new(),
            shorthand_path: "".to_string(),
            error_image_file: "".to_string(),
            missing_longhand: false,
            missing_shorthand: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return
        }

        self.shorthand_painting.show(ctx);

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
            ui.horizontal(|ui | {
                self.display_file_drop(ui);
                ui.label("OR");
                if(ui.button("Draw the shorthand").clicked()) {
                    self.shorthand_painting.activate();
                }

            });
            if self.missing_shorthand {
                ui.label("Please select a shorthand file");
            }

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    // check that the text field is not empty
                    if self.longhand.is_empty() {
                        self.missing_longhand = true;
                    }
                    if  self.shorthand_path.is_empty() {
                        self.missing_shorthand = true;
                    }
                    if !self.missing_longhand && !self.missing_shorthand {
                        // save the image
                        self.save_shorthand().expect("TODO: panic message");

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

    fn display_file_drop(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if ui.button("Load an image").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.shorthand_path = path.display().to_string();
                }
            }
            let te_shorthand = egui::TextEdit::singleline(&mut self.shorthand_path).hint_text("Path to Shorthand image");
            ui.add(te_shorthand);
            ui.label(&self.error_image_file);
            // display image here
            if !self.shorthand_path.is_empty() {
                // attempt to display the image. If the file selected is not an image file, it will throw an error
                let texture_result = utils::load_image_texture(&self.shorthand_path, ui);
                match texture_result {
                    Ok(texture) => {
                        self.error_image_file = "".to_string();
                        let img_size = 160.0 * texture.size_vec2() / texture.size_vec2().y;
                        ui.image(&texture, img_size);

                    }
                    Err(error) => {self.error_image_file = error.to_string();}
                }
            }
        });
    }

    fn save_shorthand(&mut self) -> std::io::Result<u64> {
        let path_to_save = format!("{}/{}.png", self.dict_location, self.longhand);
        // check that the file is not already in the dict location

        // copy the location to the dict location
        fs::copy(&self.shorthand_path, path_to_save)
    }

    fn reset_window(&mut self) {
        self.shorthand_painting.reset();
        self.longhand = "".to_string();
        self.missing_longhand = false;
        self.missing_shorthand = false;
        self.shorthand_path = "".to_string();
        self.error_image_file = "".to_string();
    }

    pub fn activate(&mut self) {
        self.open = true
    }
}

