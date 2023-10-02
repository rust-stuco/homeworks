use std::fmt::Display;

struct Image;

enum PageContent {
    Blank,
    Heading(String),
    Text(String),
    Picture(Image),
}

impl Display for PageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PageContent::Blank => write!(f, "\n\n"),
            PageContent::Heading(heading) => write!(f, "\n{}\n", heading.to_uppercase()),
            PageContent::Text(s) => write!(f, "{}", s),
            PageContent::Picture(_) => write!(f, "\n< Insert Image Here >\n"),
        }
    }
}

struct Page {
    front: Vec<PageContent>,
    back: Vec<PageContent>,
}

pub struct Book {
    pub title: String,
    pub author: String,
    length: usize,
    left_pages: Vec<Page>,
    right_pages: Vec<Page>,
}

impl Book {
    fn new(title: &str, author: &str, length: usize) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            length,
            left_pages: Vec::with_capacity((length + 1) / 2),
            right_pages: Vec::with_capacity((length + 1) / 2),
        }
    }

    fn size(&self) -> usize {
        self.length
    }

    fn read_left(&self) -> Option<String> {
        todo!()
    }

    fn read_right(&self) -> Option<String> {
        todo!()
    }

    /// If already at the end, do nothing
    fn flip_right(&mut self) {
        todo!()
    }

    /// If already at the beginning, do nothing
    fn flip_left(&mut self) {
        todo!()
    }
}
