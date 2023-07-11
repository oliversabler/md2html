use std::env;
use std::fs;
use std::path::Path;

mod file_writer;
mod html_formatter;
mod markdown_parser;

use file_writer::FileWriter;
use html_formatter::HtmlFormatter;
use markdown_parser::MarkdownParser;

fn setup_folder() {
    let posts_folder = Path::new("posts");
    if !posts_folder.exists() {
        fs::create_dir(posts_folder).expect("Failed to create the 'posts' folder");
    }
}

fn main() {
    setup_folder();

    let args: Vec<String> = env::args().collect();

    let markdown_file_path = args.get(1).expect("Please provide a Markdown file path");
    let html_title = args.get(2).expect("Please provide a HTML title.");

    let markdown_content =
        fs::read_to_string(markdown_file_path).expect("Failed to read the markdown file");

    let parser = MarkdownParser::new();
    let html_content = parser.parse(&markdown_content, &html_title);
    let output_file_path = format!(
        "posts/{}.html",
        Path::new(markdown_file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
    );

    let writer = FileWriter::new();
    writer.write(&html_content, &output_file_path);

    let formatter = HtmlFormatter::new();
    formatter.format(output_file_path);
}
