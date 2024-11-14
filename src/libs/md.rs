use pulldown_cmark::{Parser, Options, html};
use std::collections::HashMap;

// read frontmatter, return a map
pub fn read_frontmatter(markdown: &str) -> HashMap<String, String> {
    let frontmatter = markdown.split("---").nth(1).unwrap().to_string();
    // if frontmatter is empty or only one line, panic
    if frontmatter.is_empty() || frontmatter.split("\n").count() < 2 {
        panic!("Frontmatter is empty or only one line");
    }
    let mut frontmatter_map = HashMap::new();
    frontmatter.split("\n").for_each(|line| {
        let parts = line.split(":").collect::<Vec<&str>>();
        if parts.len() < 2 {
            return;
        }
        frontmatter_map.insert(parts[0].to_string().trim().to_string(), parts[1].to_string().trim().to_string());
    });
    frontmatter_map
}

// convert markdown to html
pub fn markdown_to_html(markdown: &str) -> String {
    // parse options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // create parser
    let parser = Parser::new_ext(markdown, options);
    
    // convert parsed content to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}
