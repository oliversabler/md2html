use std::fs;
use std::io::Write;

pub struct FileWriter;

impl FileWriter {
    pub fn new() -> Self {
        FileWriter
    }

    pub fn write(&self, content: &str, file_path: &str) {
        let mut file = fs::File::create(file_path).expect("Failed to create the output file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to the output file");
    }
}
