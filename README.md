# post_config
**The project is under development.**

This simple Rust command-line tool is developed for personal use, aimed at assisting in the creation of blog posts for my website. It handles input such as the post's title, date, tags, and categories, and generates a Markdown file named `{date}-{title}.md`. This file includes YAML formatted metadata, making each post recognizable and properly formatted for my blogging platform.

## TODO

- [ ] tags/categories

## Usage

```bash
post_config 0.1.0

USAGE:
    post_config.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --categories <categories>...    Categories of the article
        --date <date>                   Publication date of the article (optional)
        --output-dir <output-dir>        [default: .]
        --tags <tags>...                Tags associated with the article
        --title <title>                 Title of the article [default: ]
```

## Issues

- TextEdit cannot display Chinese characters correctly.