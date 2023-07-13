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

        let content = markdown::to_html(markdown_content);

        let postcontent = "</body>\n</html>".to_string();

        format!("{} {} {}", precontent, content, postcontent)
    }
}
