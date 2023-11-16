#![allow(dead_code)]

struct DoubleSidedPage {
    front: String,
    back: String,
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
    left_pages: Vec<DoubleSidedPage>,
    right_pages: Vec<DoubleSidedPage>,
}

impl Book {
    /// Creates an empty book given a title and an author
    fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            left_pages: Vec::new(),
            right_pages: Vec::new(),
        }
    }

    /// Returns the number of pages in the book
    fn size(&self) -> usize {
        // 2 extra for the front and back cover
        self.left_pages.len() + self.right_pages.len() + 2
    }

    /// Read the left page of where the book is opened to
    /// If the book is at the very beginning, return the title of the book
    fn read_left_page(&self) -> &str {
        if self.left_pages.is_empty() {
            &self.title
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the first elem
            &self.left_pages.last().unwrap().back
        }
    }

    /// Read the right page of where the book is opened to
    /// If the book is at the very end, return "The End" as a string
    fn read_right_page(&self) -> &str {
        if self.right_pages.is_empty() {
            "The End"
        } else {
            // SAFETY: We check if the vector is empty, so we are fine to unwrap the first elem
            &self.right_pages.last().unwrap().front
        }
    }

    /// Insert a DoubleSidedPage into the book
    /// Always inserts wherever the book is open on the right side
    fn insert_page(&mut self, new_page: DoubleSidedPage) {
        self.right_pages.push(new_page);
    }

    // TODO Maybe add a remove page?

    /// Turn the book to the right to see the next two pages
    /// If already at the end, return false, otherwise return true
    fn next_pages(&mut self) -> bool {
        match self.right_pages.pop() {
            None => false,
            Some(page) => {
                self.left_pages.push(page.flip());
                true
            }
        }
    }

    /// Turn the book to the left to see the previous two pages
    /// If already at the beginning, return false, otherwise return true
    fn prev_pages(&mut self) -> bool {
        match self.left_pages.pop() {
            None => false,
            Some(page) => {
                self.right_pages.push(page.flip());
                true
            }
        }
    }
}
