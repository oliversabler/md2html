# MD2HTML 
This is a simple CLI application that parses a Markdown file and converts it to a HTML file.

This is not a serious project and was created for personal use. But still, if you see 
anything that can be improved, feel free to contribute or create an issue.

## Installation
To install, run `cargo build --release` and then execute `./md2html` from the `/target/release/` folder.

If you prefer, you can bind the path to an alias to run it from other folders.

## Dependencies
To format the generated HTML we use [html-tidy](https://www.html-tidy.org/). This is executed 
automatically right before writing the HTML to disk.

Note! This has only been tested using the brew formulae of [tidy-html5](https://formulae.brew.sh/formula/tidy-html5)

## How to use
To use, run `md2html [OPTIONS] --path <PATH>`. To see other options run `md2html --help`
