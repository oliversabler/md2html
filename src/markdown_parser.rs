pub struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        MarkdownParser
    }

    pub fn parse(&self, markdown_content: &str, title: &str) -> String {
        let precontent = format!(
            "<!DOCTYPE html>
            <html>
            <head>
                <title>{}</title>
            </head>
            <body>",
            title
        );

        let content = markdown::to_html(markdown_content).to_string();

        let postcontent = format!(
            "</body>
            </html>"
        );

        format!("{} {} {}", precontent, content, postcontent)
    }
}
