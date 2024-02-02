use chrono::NaiveDate;
use eframe::egui::{self, Slider};
use egui::widgets::Widget;
use egui_extras::DatePickerButton;
use log::warn;
use std::path::PathBuf;

use crate::article_config::ArticleConfig;

pub struct MyApp {
    article: ArticleConfig,
    output_directory: PathBuf,
    date: NaiveDate,
    category_buffer: String,
    category_index: i32,
    tag_buffer: String,
    tag_index: i32,
}

impl MyApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        article: ArticleConfig,
        output_dir: PathBuf,
    ) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let date = article.get_date().unwrap();
        Self {
            article: article,
            output_directory: output_dir,
            date: date,
            category_buffer: String::default(),
            category_index: 0,
            tag_buffer: String::default(),
            tag_index: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Configure my blog posts.");

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(format!(
                    "Output directory: {}",
                    self.output_directory.display()
                ));
                if ui.button("Choose ...").clicked() {
                    if let Some(new_dir) = rfd::FileDialog::new().pick_folder() {
                        self.output_directory = new_dir;
                    }
                }
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                let title_label = ui.label("Title: ");
                ui.text_edit_singleline(&mut self.article.get_title_buffer())
                    .labelled_by(title_label.id);
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Date: ");
                DatePickerButton::new(&mut self.date).ui(ui);
            });

            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Categories: ".to_owned() + &self.article.categories_to_string());
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                let label = ui.label("New category: ");
                ui.text_edit_singleline(&mut self.category_buffer)
                    .labelled_by(label.id);
                if ui.button("Add").clicked() {
                    if !self.category_buffer.is_empty() {
                        if let Err(e) = self.article.add_category(self.category_buffer.clone()) {
                            warn!("{}", e);
                        };
                    }
                };
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Delete category: ");
                let maximum_index = self.article.categories_len() as i32 - 1;
                Slider::new(&mut self.category_index, 0..=maximum_index).ui(ui);
                if ui.button("Delete").clicked() {
                    if let Err(e) = self.article.delete_category(self.category_index as usize) {
                        warn!("{}", e);
                    };
                }
                self.category_index = 0;
            });

            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Tags: ".to_owned() + &self.article.tags_to_string());
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                let label = ui.label("New tag: ");
                ui.text_edit_singleline(&mut self.tag_buffer)
                    .labelled_by(label.id);
                if ui.button("Add").clicked() {
                    if !self.tag_buffer.is_empty() {
                        if let Err(e) = self.article.add_tag(self.tag_buffer.clone()) {
                            warn!("{}", e);
                        };
                    }
                };
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Delete tag: ");
                let maximum_index = self.article.tags_len() as i32 - 1;
                Slider::new(&mut self.tag_index, 0..=maximum_index).ui(ui);
                if ui.button("Delete").clicked() {
                    if let Err(e) = self.article.delete_tag(self.tag_index as usize) {
                        warn!("{}", e);
                    };
                }
                self.tag_index = 0;
            });

            if ui.button("Save").clicked() {
                self.article.set_date(&self.date);
                // info!("{}", self.date.format("%Y-%m-%d").to_string());
                match self.article.save(&self.output_directory) {
                    Ok(_) => (),
                    Err(e) => warn!("Failed to save article: {}", e),
                }
            }
        });
    }
}

// https://github.com/emilk/egui/blob/master/examples/custom_font/
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSerifSC-Regular.otf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
