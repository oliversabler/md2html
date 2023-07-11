pub struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        MarkdownParser
    }

    pub fn parse(&self, markdown_content: &str) -> String {
        let precontent = format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <title>{}</title>
            </head>
            <body>"#, "SOME TITLE HERE");

        let content = markdown::to_html(markdown_content).to_string();

        let postcontent = format!(
            r#"</body>
            </html>"#);

        format!("{} {} {}", precontent, content, postcontent)
    }
}
