use mdbook::book::Book;
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;

pub struct ImageViewerPreprocessor;

impl Preprocessor for ImageViewerPreprocessor {
    fn name(&self) -> &str {
        "image-viewer"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                let mut new_content = String::new();
                new_content.push_str(include_str!("css_template.html"));
                new_content.push('\n');
                
                new_content.push_str(&self.process_chapter(&chapter.content));
                new_content.push('\n');
                
                new_content.push_str(include_str!("script_template.html"));
                chapter.content = new_content;
            }
        });
        Ok(book)
    }
}

impl ImageViewerPreprocessor {
    fn process_chapter(&self, content: &str) -> String {
        let img_regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
        let processed_content = img_regex.replace_all(content, |caps: &regex::Captures| {
            let alt_text = &caps[1];
            let image_path = &caps[2];
            format!(
                r#"<img src="{}" alt="{}" class="miv_mdbook-image-viewer" onclick="miv_openModal(this.src)">"#,
                image_path, alt_text
            )
        });
        processed_content.to_string()
    }
}