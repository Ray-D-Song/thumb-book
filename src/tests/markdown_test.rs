use crate::libs::md::{markdown_to_html, read_frontmatter};

#[test]
fn test_basic_markdown() {
    let markdown = "# Hello World\nThis is a **bold** text.";
    let expected = "<h1>Hello World</h1>\n<p>This is a <strong>bold</strong> text.</p>\n";
    assert_eq!(markdown_to_html(markdown), expected);
}

#[test]
fn test_complex_markdown() {
    let markdown = r#"
# Title

## Subtitle

- List item 1
- List item 2

1. Numbered item 1
2. Numbered item 2

> This is a quote

```rust
fn hello() {
    println!("Hello, world!");
}
```

[Link](https://www.example.com)

| Table | Header |
|-------|--------|
| Cell  | Cell   |
"#;
    let result = markdown_to_html(markdown);
    
    // 验证转换后的HTML包含预期的元素
    assert!(result.contains("<h1>Title</h1>"));
    assert!(result.contains("<h2>Subtitle</h2>"));
    assert!(result.contains("<ul>"));
    assert!(result.contains("<ol>"));
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("<pre><code class=\"language-rust\">"));
    assert!(result.contains("<table>"));
    assert!(result.contains("<a href=\"https://www.example.com\">"));
}

#[test]
fn test_task_list() {
    let markdown = "- [ ] Unchecked task\n- [x] Checked task";
    let result = markdown_to_html(markdown);
    assert!(result.contains("type=\"checkbox\""));
    assert!(result.contains("checked=\"\""));
}

#[test]
fn test_empty_input() {
    assert_eq!(markdown_to_html(""), "");
}

#[test]
fn test_read_frontmatter() {
    let markdown = r#"
---
title: Hello World
date: 2024-01-01
---
# Title
"#;
    let result = read_frontmatter(markdown);
    assert_eq!(result.get("title"), Some(&String::from("Hello World")));
    assert_eq!(result.get("date"), Some(&String::from("2024-01-01")));
}