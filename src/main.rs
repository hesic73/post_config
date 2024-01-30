#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(absolute_path)]
use chrono::{Local, NaiveDate};
use eframe::egui::{self, Slider};
use egui::widgets::Widget;
use egui_extras::DatePickerButton;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    /// Title of the article
    #[structopt(long, default_value = "")]
    title: String,

    /// Publication date of the article (optional)
    #[structopt(long)]
    date: Option<String>,

    /// Tags associated with the article
    #[structopt(long, use_delimiter = true)]
    tags: Vec<String>,

    /// Categories of the article
    #[structopt(long, use_delimiter = true)]
    categories: Vec<String>,

    #[structopt(long, default_value = ".", parse(from_os_str))]
    output_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    date: String,
    categories: Vec<String>,
    tags: Vec<String>,
}

fn save_article(article: &Article, output_dir: &PathBuf) -> std::result::Result<(), String> {
    if article.title.is_empty() {
        return Err("Title is empty.".to_string());
    }
    let filename = format!("{}-{}.md", article.date, article.title.replace(" ", "-"));

    let file_path = output_dir.join(filename);
    println!("{}", file_path.display());
    if file_path.exists() {
        let s = format!("The file at {} already exists.", file_path.display());
        return Err(s);
    }
    let yaml = serde_yaml::to_string(&article).unwrap();

    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(b"---\n").expect("Unable to write to file");
    file.write_all(yaml.as_bytes())
        .expect("Unable to write to file");
    file.write_all(b"\n---\n").expect("Unable to write to file");

    info!("Article saved to {}", file_path.display());
    return Ok(());
}

fn main() -> Result<(), eframe::Error> {
    let mut builder = env_logger::Builder::from_default_env();
    // Set the default log level programmatically
    builder.filter(None, log::LevelFilter::Info);
    builder.init();

    let args = Args::from_args();

    let date = match args.date {
        Some(d) => {
            // Validate the date format
            NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                .expect("Provided date is not in the valid format (yyyy-mm-dd)")
        }
        None => {
            // Fetch the current date in yyyy-mm-dd format
            Local::now().date_naive()
        }
    };

    let output_dir = std::path::absolute(args.output_dir).unwrap();

    let article = Article {
        title: args.title.clone(),
        date: date.format("%Y-%m-%d").to_string(),
        tags: args.tags,
        categories: args.categories,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Post Configuration",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp::new(_cc, article, output_dir))),
    )
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

struct MyApp {
    article: Article,
    output_directory: PathBuf,
    date: NaiveDate,
    category_buffer: String,
    category_index: i32,
    tag_buffer: String,
    tag_index: i32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>, article: Article, output_dir: PathBuf) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let date = NaiveDate::parse_from_str(&article.date, "%Y-%m-%d").unwrap();
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
                ui.text_edit_singleline(&mut self.article.title)
                    .labelled_by(title_label.id);
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Date: ");
                DatePickerButton::new(&mut self.date).ui(ui);
            });

            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Categories: ");
                for category in self.article.categories.iter() {
                    ui.label(category);
                }
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                let label = ui.label("New category: ");
                ui.text_edit_singleline(&mut self.category_buffer)
                    .labelled_by(label.id);
                if ui.button("Add").clicked() {
                    if !self.category_buffer.is_empty() {
                        for category in self.article.categories.iter() {
                            if *category == self.category_buffer {
                                return;
                            }
                        }
                        self.article.categories.push(self.category_buffer.clone());
                    }
                };
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Delete category: ");
                let maximum_index = self.article.categories.len() as i32 - 1;
                Slider::new(&mut self.category_index, 0..=maximum_index).ui(ui);
                if ui.button("Delete").clicked() {
                    if self.category_index < 0
                        || self.category_index >= self.article.categories.len() as i32
                    {
                        return;
                    }
                    self.article.categories.remove(self.category_index as usize);
                }
                self.category_index = 0;
            });

            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Tags: ");
                for tag in self.article.tags.iter() {
                    ui.label(tag);
                }
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                let label = ui.label("New tag: ");
                ui.text_edit_singleline(&mut self.tag_buffer)
                    .labelled_by(label.id);
                if ui.button("Add").clicked() {
                    if !self.tag_buffer.is_empty() {
                        for tag in self.article.tags.iter() {
                            if *tag == self.tag_buffer {
                                return;
                            }
                        }
                        self.article.tags.push(self.tag_buffer.clone());
                    }
                };
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Delete tag: ");
                let maximum_index = self.article.tags.len() as i32 - 1;
                Slider::new(&mut self.tag_index, 0..=maximum_index).ui(ui);
                if ui.button("Delete").clicked() {
                    if self.tag_index < 0 || self.tag_index >= self.article.tags.len() as i32 {
                        return;
                    }
                    self.article.tags.remove(self.tag_index as usize);
                }
                self.tag_index = 0;
            });

            if ui.button("Save").clicked() {
                self.article.date = self.date.format("%Y-%m-%d").to_string();
                // info!("{}", self.date.format("%Y-%m-%d").to_string());
                match save_article(&self.article, &self.output_directory) {
                    Ok(_) => (),
                    Err(e) => warn!("Failed to save article: {}", e),
                }
            }
        });
    }
}
