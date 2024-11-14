use crate::libs::process::Post;

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
