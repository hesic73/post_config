#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(absolute_path)]
use eframe::egui;

use chrono::{Local, NaiveDate};
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
                .format("%Y-%m-%d")
                .to_string()
        }
        None => {
            // Fetch the current date in yyyy-mm-dd format
            Local::now().format("%Y-%m-%d").to_string()
        }
    };

    let output_dir = std::path::absolute(args.output_dir).unwrap();

    let article = Article {
        title: args.title.clone(),
        date: date.clone(),
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
        Box::new(|_cc| {
            Box::<MyApp>::new(MyApp {
                article: article,
                output_directory: output_dir,
            })
        }),
    )
}

struct MyApp {
    article: Article,
    output_directory: PathBuf,
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

            if ui.button("Save").clicked() {
                match save_article(&self.article, &self.output_directory) {
                    Ok(_) => info!("Article saved successfully"),
                    Err(e) => warn!("Failed to save article: {}", e),
                }
            }
        });
    }
}
