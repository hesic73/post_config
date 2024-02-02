#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(absolute_path)]
mod app;
mod article_config;
mod my_text_buffer;
use app::MyApp;
use article_config::ArticleConfig;
use chrono::{Local, NaiveDate};
use eframe::egui;
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

    let article = ArticleConfig::new(
        args.title,
        date.format("%Y-%m-%d").to_string(),
        args.tags,
        args.categories,
    );

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
