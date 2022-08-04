use eframe::egui;
use eframe::Frame;
use egui::*;
use crate::add_new_word_window::AddNewWordWindow;
use crate::model::shorthand_dict::ShorthandDict;
use crate::shorthand_dict;
use crate::shorthand_dict::Longhand;
use crate::add_new_word_window;
use crate::utils;



#[derive(Default)]
pub struct ShorthandApp {
    location: String,
    dict: ShorthandDict,
    selected: Option<Longhand>,
    searched: String,
    add_edit_word_window: AddNewWordWindow
}

impl ShorthandApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // TODO change this to actual loading screen
        let loc = "test/resources/load".to_string();
        let my_dict = shorthand_dict::load(&loc);
        match my_dict {
            Ok(tl_dict) => {
                ShorthandApp {
                location: loc.clone(),
                dict: tl_dict,
                selected: None,
                searched: "".to_string(),
                add_edit_word_window: AddNewWordWindow::new(loc.clone())
            }}
            Err(_) => {todo!()}
        }

    }

    fn show_search_bar(&mut self, ui: &mut Ui) {
        ui.add(egui::TextEdit::singleline(&mut self.searched).hint_text("Search"));
    }

    fn show_word_management_bar(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // contains buttons to add new words, edit, delete words
        if(ui.add(egui::Button::new("Add")).clicked()) {
            self.add_edit_word_window.activate()
        };
    }

    fn add_word_list(&mut self, ui: &mut Ui) {
        // load all the words with one label for each
        // if self.searched is Some(_), only load the words that contain the searched text
        for item in &self.dict {
                if self.searched.eq("") {
                    if ui.add(SelectableLabel::new(false, item.0)).clicked() {
                        self.selected = Some(item.0.to_string());
                    }
                }
                else {
                    if item.0.contains(&self.searched) {
                        if ui.add(SelectableLabel::new(false, item.0)).clicked() {
                            self.selected = Some(item.0.to_string());
                        }
                    }
                }
        }
    }

    fn show_shorthand(&mut self, ui : &mut Ui) {
        match &self.selected {
            None => {
                // display nothing
            }
            Some(longhand) => {
                // load the image corresponding
                let path = format!("{}/{}.png", &self.location, longhand);

                // and display it here
                let texture = utils::load_image_texture(&path, ui);
                match texture {
                    Ok(t) => {
                        let img_size = 160.0 * t.size_vec2() / t.size_vec2().y;
                        ui.image(&t, img_size);}
                    Err(_) => {todo!();}
                }
            }
        };
    }
}


impl eframe::App for ShorthandApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::containers::TopBottomPanel::top("my_top_panel").show(ctx, |ui| {
            self.show_search_bar(ui);
            self.show_word_management_bar(ctx, ui);
        });
        // Put the list of words on the left paner
        SidePanel::left("Test side panel").show(ctx, |ui| {
            self.add_word_list(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            self.show_shorthand(ui);
        });

        // show the various additional windows. Whether they are active or not is covered in their implementation
        self.add_edit_word_window.show(ctx);

    }

}

