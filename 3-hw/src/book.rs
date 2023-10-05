use std::fmt::Display;

struct Image {
    _width: usize,
    _height: usize,
    caption: String,
}

enum PageContent {
    Blank,
    Heading(String),
    Text(String),
    /// Magical pages that can contain any image
    Picture(Image),
}

impl Display for PageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PageContent::Blank => write!(f, "\n\n"),
            PageContent::Heading(heading) => write!(f, "\n{}\n", heading.to_uppercase()),
            PageContent::Text(s) => write!(f, "{}", s),
            PageContent::Picture(img) => write!(f, "\n< {} >\n", img.caption),
        }
    }
}

struct Page(Vec<PageContent>);

struct DoubleSidedPage {
    front: Page,
    back: Page,
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Page Content: {{")?;
        for page_content in &self.0 {
            write!(f, "{}", page_content)?;
        }
        writeln!(f, "}}")
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
    length: usize,
    left_pages: Vec<DoubleSidedPage>,
    right_pages: Vec<DoubleSidedPage>,
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
        if self.left_pages.is_empty() {
            None
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the last elem
            Some(format!("{}", self.left_pages.last().unwrap().back))
        }
    }

    fn read_right(&self) -> Option<String> {
        if self.right_pages.is_empty() {
            None
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the first elem
            Some(format!("{}", self.right_pages.last().unwrap().front))
        }
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
