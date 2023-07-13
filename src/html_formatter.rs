use std::process::Command;

pub struct HtmlFormatter;

impl HtmlFormatter {
    pub fn new() -> Self {
        HtmlFormatter
    }

    pub fn format(&self, path: String) {
        Command::new("tidy")
            .arg("-m")
            .arg("-i")
            .arg("-clean")
            .arg("yes")
            .arg("--tidy-mark")
            .arg("no")
            .arg(path)
            .output()
            .expect("Unable to format HTML using Tidy.");
    }
}
