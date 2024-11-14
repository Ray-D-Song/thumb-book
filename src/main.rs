mod libs;
use libs::process::process_posts;
pub use libs::md::markdown_to_html;
use std::path::Path;
use std::env::current_dir;
use libs::generate::generate_routes;
use libs::generate::generate_search_index;
use libs::process::process_index;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 && args[1] == "init" {
        libs::init::init();
        return;
    }

    let current_dir = current_dir().unwrap();
    libs::check::check_dir().unwrap();
    let posts_dir = Path::new(&current_dir).join("posts");
    let posts_output_dir = Path::new(&current_dir).join("dist").join("posts");

    // if posts_output_dir exists, delete it
    let posts_output_dir_clone = posts_output_dir.clone();
    if posts_output_dir_clone.exists() {
        std::fs::remove_dir_all(posts_output_dir_clone).unwrap();
    }
    
    // create posts_output_dir
    let posts_output_dir_clone = posts_output_dir.clone();
    std::fs::create_dir_all(posts_output_dir_clone).unwrap();

    // process posts
    let posts_vec = process_posts(&posts_dir.to_str().unwrap(), &posts_output_dir.to_str().unwrap());
    let posts = posts_vec.unwrap();
    // generate routes
    let routes = generate_routes(&posts).unwrap();
    // process index
    let output_index_path = Path::new(&current_dir).join("dist").join("index.html");
    let _ = process_index(&current_dir.join("index.md").to_str().unwrap(), &output_index_path.to_str().unwrap(), &routes);
    // generate search index
    generate_search_index(&posts).unwrap();

    println!("Done!");
}

#[cfg(test)]
mod tests;