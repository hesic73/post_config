use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::my_text_buffer::MyTextBuffer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleConfig {
    title: String,
    date: String,
    categories: Vec<String>,
    tags: Vec<String>,
}

impl ArticleConfig {
    pub fn new(title: String, date: String, categories: Vec<String>, tags: Vec<String>) -> Self {
        return ArticleConfig {
            title: title,
            date: date,
            categories: categories,
            tags: tags,
        };
    }

    pub fn save(self: &mut Self, output_dir: &PathBuf) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title is empty.".to_string());
        }
        let filename = format!("{}-{}.md", self.date, self.title.replace(" ", "-"));

        let file_path = output_dir.join(filename);
        println!("{}", file_path.display());
        if file_path.exists() {
            let s = format!("The file at {} already exists.", file_path.display());
            return Err(s);
        }
        let yaml = serde_yaml::to_string(&self).unwrap();

        let mut file = File::create(&file_path).expect("Unable to create file");
        file.write_all(b"---\n").expect("Unable to write to file");
        file.write_all(yaml.as_bytes())
            .expect("Unable to write to file");
        file.write_all(b"\n---\n").expect("Unable to write to file");

        info!("ArticleConfig saved to {}", file_path.display());
        return Ok(());
    }

    pub fn get_date(self: &Self) -> Result<NaiveDate, String> {
        match NaiveDate::parse_from_str(&self.date, "%Y-%m-%d") {
            Ok(date) => Ok(date),
            Err(parse_error) => Err(parse_error.to_string()),
        }
    }

    pub fn set_date(self: &mut Self, date: &NaiveDate) {
        self.date = date.format("%Y-%m-%d").to_string();
    }

    pub fn get_title_buffer(self: &mut Self) -> MyTextBuffer {
        return MyTextBuffer::new(&mut self.title);
    }

    pub fn tags_to_string(self: &Self) -> String {
        return self.tags.join(" ");
    }
    pub fn categories_to_string(self: &Self) -> String {
        return self.categories.join(" ");
    }

    pub fn add_tag(self: &mut Self, tag: String) -> Result<(), String> {
        for _tag in self.tags.iter() {
            if *_tag == tag {
                return Err(format!("Tag {} already exist.", tag));
            }
        }
        self.tags.push(tag);

        Ok(())
    }
    pub fn add_category(self: &mut Self, category: String) -> Result<(), String> {
        for _category in self.categories.iter() {
            if *_category == category {
                return Err(format!("Category {} already exist.", category));
            }
        }
        self.categories.push(category);

        Ok(())
    }

    pub fn delete_tag(self: &mut Self, index: usize) -> Result<(), String> {
        if self.tags.is_empty() {
            return Err("There is no tag yet!".to_string());
        }
        if index >= self.tags.len() {
            return Err(format!("Index {} out of bound!", index));
        }
        self.tags.remove(index);
        Ok(())
    }

    pub fn delete_category(self: &mut Self, index: usize) -> Result<(), String> {
        if self.categories.is_empty() {
            return Err("There is no category yet!".to_string());
        }
        if index >= self.categories.len() {
            return Err(format!("Index {} out of bound!", index));
        }
        self.categories.remove(index);
        Ok(())
    }

    pub fn tags_len(self: &Self) -> usize {
        return self.tags.len();
    }
    pub fn categories_len(self: &Self) -> usize {
        return self.categories.len();
    }
}
