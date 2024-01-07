//! This module contains the [`EmailReader`] type, as well as its method implementations.
//! The [`EmailReader`] struct will model an email message.
//! [`EmailReader`] should have the following attributes:
//!  - `subject: String`
//!  - `from: String`
//!  - `to: String`
//!  - `message: String`
//!
//! All of these fields should be _private_ (not accessible outside of the struct).
//!
//! ---
//!
//! Once you've added the fields to the struct, implement the following methods:
//! - [`parse`](EmailReader::parse): This method will take in a file path and create a new [`EmailReader`] based on the file.
//!
//! ---
//!
//! We also want [`EmailReader`]s to be able to summarize themselves.
//! We'll implement the following methods
//! [`summarize`](Summary::summarize), [`get_info`](Summary::get_info), and [`msg_len`](Summary::msg_len).
//!
//! [`summarize`](Summary::summarize) should return a string that contains `"<sender>: message"`.
//!
//! [`get_info`](Summary::get_info) should return a string that contains `"<subject>\n From: <sender>, To: <receiver>"`.
//!
//! [`msg_len`](Summary::msg_len) should return the length of the message.
//!

use crate::summarize::Summary;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// EmailReader is a struct that represents an email message.
pub struct EmailReader {
    subject: String,
    from: String,
    to: String,
    message: String,
}

impl EmailReader {
    /// Creates a new EmailReader
    /// `file_path` - is the path to the text message that will be read.
    ///
    /// The file should be in the following format:
    ///
    /// Subject: \<subject\>\
    /// From: \<sender\>\
    /// \<message\>\
    /// To: \<receiver\>
    ///
    /// Use the `std::fs` and `std::io` libraries to read the file
    /// and get the sender, receiver, subject and message
    ///
    /// If the file does not exist, panic with the message "File not found"
    /// If the file is not in the correct format, panic with the message "File is not in correct format"
    /// If no message is found, assume the message is ""
    pub fn parse(file_path: String) -> Result<EmailReader, std::io::Error> {
        let file = File::open(file_path).expect("File not found");

        let mut from = String::new();
        let mut message = String::from("");
        let mut to = String::new();
        let mut subject = String::new();

        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for (num, line) in reader.lines().enumerate() {
            let line: String = line?;

            let components = line.split(": ").collect::<Vec<&str>>();

            if num == 2 && components.len() == 1 {
                message = line;
                continue;
            }

            if components.len() != 2 {
                panic!("File is not in correct format");
            }

            let title = components[0];
            let content = components[1];

            match title {
                "Subject" => subject = content.to_string(),
                "From" => from = content.to_string(),
                "To" => to = content.to_string(),
                _ => panic!("File is not in correct format"),
            }
        }

        Ok(EmailReader {
            subject,
            from,
            to,
            message,
        })
    }
}

impl Summary for EmailReader {
    fn summarize(&self) -> String {
        format!("{}: {}", self.from, self.message)
    }

    fn get_info(&self) -> String {
        format!("{}\n From: {}, To {}", self.subject, self.from, self.to)
    }

    fn msg_len(&self) -> usize {
        self.message.len()
    }
}
