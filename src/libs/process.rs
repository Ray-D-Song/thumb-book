use std::path::Path;
use crate::libs::md::{read_frontmatter, markdown_to_html};
use crate::libs::config::read_config;

const INDEX_TEMPLATE: &str = include_str!("../../static/template_index.html");
const POST_TEMPLATE: &str = include_str!("../../static/template_post.html");

#[derive(Debug)]
#[derive(Clone)]
pub struct Post {
    pub path: String,
    pub title: String,
    pub date: String,
}

pub fn process_posts(posts_dir: &str, posts_output_dir: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let posts = std::fs::read_dir(posts_dir)?;
    let mut posts_vec = Vec::new();
    
    for post in posts {
        let post = post?;
        let post_path = post.path();
        let post_path_clone = post_path.clone();
        let post_content = std::fs::read_to_string(post_path_clone)?;
        
        let frontmatter = read_frontmatter(&post_content);
        
        let post_info = Post {
            path: format!("/posts/{}", post_path.file_name().unwrap().to_str().unwrap()),
            title: frontmatter.get("title").unwrap_or(&String::from("Untitled")).clone(),
            date: frontmatter.get("date").unwrap_or(&String::from("")).clone(),
        };
        let post_info_clone = post_info.clone();
        posts_vec.push(post_info_clone);

        let post_content = post_content.split("---").nth(2).unwrap();

        let html = markdown_to_html(post_content);

        let output = POST_TEMPLATE
            .replace("{{title}}", &post_info.title)
            .replace("{{date}}", &post_info.date)
            .replace("{{content}}", &html);

        let file_name = post_path.file_name().unwrap().to_str().unwrap();
        let html_file_name = file_name.replace(".md", ".html");
        let output_path = Path::new(posts_output_dir).join(html_file_name);
        std::fs::write(output_path, output)?;
    }
    
    Ok(posts_vec)
}

pub fn process_index(index_path: &str, output_index_path: &str, routes: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    
    let config = read_config("config.toml")?;
    let index = std::fs::read_to_string(index_path)?;
    let html = markdown_to_html(&index);
    let output = INDEX_TEMPLATE
      .replace("{{routes}}", &routes.join("\n"))
      .replace("{{content}}", &html)
      .replace("{{title}}", &config.get("title").unwrap());

    std::fs::write(output_index_path, output)?;
    Ok(())
}
