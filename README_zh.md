## thumb-book
最小化静态网站生成器，由 Rust 编写。

实现路径：
- [x] markdown 转 html
- [x] 读取根目录的配置文件
- [x] 写入默认配置文件
- [x] 验证模版替换

处理流程：
1. 读取根目录的配置文件
2. 读取 posts 目录下的 markdown 文件, 替换{{content}}生成 html, 在处理每个 markdown 文件时, 需要读取文件中的 frontmatter 数据写入变量中
3. 通过 frontmatter 生成的数据，生成文章列表，写入到 index.html 中