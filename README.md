# post_config
This simple Rust command-line tool is developed for personal use, aimed at assisting in the creation of blog posts for my website. It handles input such as the post's title, date, tags, and categories, and generates a Markdown file named `{date}-{title}.md`. This file includes YAML formatted metadata, making each post recognizable and properly formatted for my blogging platform.

## Usage

```bash
USAGE:
    post_config.exe [OPTIONS] <title>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --categories <categories>...    Categories of the article
        --date <date>                   Publication date of the article (optional)
        --output-dir <output-dir>        [default: .]
        --tags <tags>...                Tags associated with the article

ARGS:
    <title>    Title of the article
```
