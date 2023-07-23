use clap::Parser;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "path", value_hint = clap::ValueHint::FilePath)]
    path: String,

    #[arg(short = 't', long = "title")]
    html_title: Option<String>,
}

pub struct Config {
    pub file_path: String,
    pub html_title: String,
}

impl Config {
    pub fn setup() -> Result<Config, &'static str> {
        if let Err(e) = create_folder() {
            panic!("Unable to create folder: {e}")
        };

        let args = Args::parse();

        let file_path = args.path;
        let html_title = match args.html_title {
            Some(title) => title,
            None => file_path.clone().drain(0..file_path.len() - 3).collect(),
        };

        Ok(Config {
            file_path,
            html_title,
        })
    }
}

fn create_folder() -> Result<(), Box<dyn Error>> {
    let posts_folder = Path::new("posts");
    if !posts_folder.exists() {
        fs::create_dir(posts_folder)?;
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let markdown_content = fs::read_to_string(&config.file_path)?;

    let html_content = parse(&config, &markdown_content)?;

    let output_file_path = format!(
        "posts/{}.html",
        Path::new(&config.file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
    );

    write(&html_content, &output_file_path)?;

    format(&output_file_path)?;

    Ok(())
}

pub fn parse(config: &Config, content: &str) -> Result<String, Box<dyn Error>> {
    let precontent = format!(
        "<!DOCTYPE html>
        <html>
        <head>
        <title>{}</title>
        </head>
        <body>",
        config.html_title
    );

    let content = markdown::to_html(content);

    let postcontent = "</body>\n</html>".to_string();

    Ok(format!("{} {} {}", precontent, content, postcontent))
}

pub fn write(content: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(output_file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn format(path: &str) -> Result<(), Box<dyn Error>> {
    Command::new("tidy")
        .arg("-m")
        .arg("-i")
        .arg("-clean")
        .arg("yes")
        .arg("--tidy-mark")
        .arg("no")
        .arg(path)
        .output()?;

    Ok(())
}
