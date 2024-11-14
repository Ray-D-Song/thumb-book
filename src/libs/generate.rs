use crate::libs::process::Post;
use serde_json::json;

pub fn generate_routes(posts_vec: &Vec<Post>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut a_tags = Vec::new();
    let mut sorted_posts = posts_vec.clone();
    
    sorted_posts.sort_by(|a, b| b.date.cmp(&a.date));
    
    for post in sorted_posts {
        let a_tag = format!(
            r#"<a href="{}" class="post-link">
                <span class="post-title">{}</span>
                <span class="post-date">{}</span>
            </a>"#,
            post.path.replace(".md", ".html"),
            post.title,
            post.date
        );
        a_tags.push(a_tag);
    }
    
    Ok(a_tags)
}

pub fn generate_search_index(posts_vec: &Vec<Post>) -> Result<(), Box<dyn std::error::Error>> {
    let mut search_index = Vec::new();
    
    for (id, post) in posts_vec.iter().enumerate() {
        let content = std::fs::read_to_string(format!("posts/{}", post.path))?;
        let search_item = json!({
            "id": id,
            "title": post.title,
            "content": content,
            "url": post.path.replace(".md", ".html")
        });
        search_index.push(search_item);
    }
    
    let json = serde_json::to_string(&search_index)?;
    std::fs::write("dist/search-index.json", json)?;
    Ok(())
}
