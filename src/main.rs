use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    /// Title of the article
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
    output_dir: std::path::PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    date: String,
    categories: Vec<String>,
    tags: Vec<String>,
}

fn main() {
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

    let article = Article {
        title: args.title.clone(),
        date: date.clone(),
        tags: args.tags,
        categories: args.categories,
    };

    let yaml = serde_yaml::to_string(&article).unwrap();

    let filename = format!("{}-{}.md", date, article.title.replace(" ", "-"));
    let file_path = args.output_dir.join(filename);
    if file_path.exists() {
        println!(
            "Warning: The file at '{}' already exists!",
            file_path.display()
        );
        return;
    }

    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(b"---\n").expect("Unable to write to file");
    file.write_all(yaml.as_bytes())
        .expect("Unable to write to file");
    file.write_all(b"\n---\n").expect("Unable to write to file");

    println!("Article saved to {}", file_path.display());
}
