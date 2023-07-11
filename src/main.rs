use std::env;
use std::fs;
use std::path::Path;

mod file_writer;
mod markdown_parser;

use file_writer::FileWriter;
use markdown_parser::MarkdownParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let markdown_file_path = args.get(1).expect("Please provide a markdown file path");

    let markdown_content =
        fs::read_to_string(markdown_file_path).expect("Failed to read the markdown file");

    let posts_folder = Path::new("posts");
    if !posts_folder.exists() {
        fs::create_dir(posts_folder).expect("Failed to create the 'posts' folder");
    }

    let parser = MarkdownParser::new();
    let writer = FileWriter::new();

    let html_content = parser.parse(&markdown_content);

    let output_file_path = format!(
        "posts/{}.html",
        Path::new(markdown_file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
    );

    writer.write(&html_content, &output_file_path);
}
