use std::fs;
use std::path::Path;

use clap::Parser;

mod file_writer;
mod html_formatter;
mod markdown_parser;

use file_writer::FileWriter;
use html_formatter::HtmlFormatter;
use markdown_parser::MarkdownParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "path", value_hint = clap::ValueHint::FilePath)]
    path: String,

    #[arg(short = 't', long = "title")]
    html_title: Option<String>,
}

fn setup_folder() {
    let posts_folder = Path::new("posts");
    if !posts_folder.exists() {
        fs::create_dir(posts_folder).expect("Failed to create the 'posts' folder");
    }
}

fn main() {
    setup_folder();
    let args = Args::parse();

    let markdown_file_path = args.path;

    let html_title = if !args.html_title.is_none() {
        args.html_title.unwrap()
    } else {
        markdown_file_path.clone()
    };

    let markdown_content =
        fs::read_to_string(&markdown_file_path).expect("Failed to read the markdown file");

    let parser = MarkdownParser::new();
    let html_content = parser.parse(&markdown_content, &html_title);
    let output_file_path = format!(
        "posts/{}.html",
        Path::new(&markdown_file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
    );

    let writer = FileWriter::new();
    writer.write(&html_content, &output_file_path);

    let formatter = HtmlFormatter::new();
    formatter.format(output_file_path);
}
