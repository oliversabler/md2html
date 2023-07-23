use md2html::Config;
use std::process;

fn main() {
    let config = Config::setup().unwrap_or_else(|err| {
        eprintln!("Problem occured when setting up the program: {err}");
        process::exit(1);
    });

    if let Err(e) = md2html::run(config) {
        eprintln!("Application parsing error: {e}");
        process::exit(1);
    }
}
