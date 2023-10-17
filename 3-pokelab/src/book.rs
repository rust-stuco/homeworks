use std::fmt::Display;

struct Image {
    width: usize,
    height: usize,
    caption: String,
}

enum Page {
    Blank,
    Text(String),
    /// These magical pages can contain any image
    Picture(Image),
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Page::Blank => writeln!(f, "\n\n\n"),
            Page::Text(s) => writeln!(f, "{}", s),
            Page::Picture(img) => {
                writeln!(f, "\n< {} >({}x{})\n", img.caption, img.width, img.height)
            }
        }
    }
}

struct DoubleSidedPage {
    front: Page,
    back: Page,
}

impl DoubleSidedPage {
    fn flip(self) -> Self {
        Self {
            front: self.back,
            back: self.front,
        }
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

    /// TODO description
    fn read_left(&self) -> Option<String> {
        if self.left_pages.is_empty() {
            None
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the last elem
            Some(format!("{}", self.left_pages.last().unwrap().back))
        }
    }

    /// TODO description
    fn read_right(&self) -> Option<String> {
        if self.right_pages.is_empty() {
            None
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the first elem
            Some(format!("{}", self.right_pages.last().unwrap().front))
        }
    }

    /// TODO description
    /// Always inserts wherever the book is open on the right side
    fn insert_page(&mut self, new_page: DoubleSidedPage) {
        self.right_pages.push(new_page);
    }

    // Maybe add a remove page?

    /// TODO description
    /// If already at the end, do nothing
    fn flip_next(&mut self) {
        match self.right_pages.pop() {
            None => (),
            Some(page) => self.left_pages.push(page.flip()),
        }
    }

    /// TODO description
    /// If already at the beginning, do nothing
    fn flip_prev(&mut self) {
        match self.left_pages.pop() {
            None => (),
            Some(page) => self.right_pages.push(page.flip()),
        }
    }
}
