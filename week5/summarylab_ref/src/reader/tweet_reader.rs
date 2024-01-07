//! This module contains the [`TweetReader`] type, as well as its method implementations.
//!
//! The [`TweetReader`] struct will model a tweet.
//! [`TweetReader`] should have the following attributes:
//! - `username: String`
//! - `content: String`
//! - `reply: bool`
//! - `retweet: bool`
//!
//! All of these fields should be _private_ (not accessible outside of the struct).
//!
//! ---
//!
//! Once you've added the fields to the struct, implement the following methods:
//! - [`parse`](TweetReader::parse): This method will take in a file path and
//! create a new [`TweetReader`] based on the file.
//!
//! ---
//!
//! We also want [`TweetReader`]s to be able to summarize themselves.
//! We'll implement the following methods
//! [`msg_len`](Summary::msg_len), [`summarize`](Summary::summarize),
//! and [`get_info`](Summary::get_info),
//!
//! - [`msg_len`](Summary::msg_len) should return the length of the message.
//! - [`summarize`](Summary::summarize) should return a string that
//! contains `"@<username>: <content>"`.
//! - [`get_info`](Summary::get_info) should return a string that contains
//! `"Tweet from @<username> "`. with (reply) or (retweet) appended.

use crate::Summary;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// TweetReader is a struct that represents a tweet.
pub struct TweetReader {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl TweetReader {
    /// Creates a new [`TweetReader`].
    ///
    /// file_path - is the path to the text message that will be read
    /// The file should be in the following format:
    /// @{username}
    /// "{content}"
    ///
    /// Content can span multiple lines. Content will be surrounded by quotes.
    /// If the tweet is a reply, the word "reply" will be on a new line after content
    /// If the tweet is a retweet, the word "retweet" will be on a new line after content or reply
    ///
    /// Use the use std::fs and std::io to read the file
    /// and get the sender, receiver, subject and message
    ///
    /// If the file does not exist, panic with the message "File not found"
    /// If the file is not in the correct format, panic with the message
    /// "File is not in correct format".
    /// If no message is found, assume the message is "".
    pub fn parse(file_path: String) -> Result<TweetReader, std::io::Error> {
        let file = File::open(file_path).expect("File not found");
        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(|line| line.unwrap());

        let username = lines.next().unwrap()[1..].to_string();

        let mut content = lines.next().unwrap();
        if content.starts_with('\"') {
            content = content[1..].to_string();
        }

        loop {
            let line = lines.next().unwrap();

            match line.find('\"') {
                Some(index) => {
                    if index == line.len() - 1 {
                        content.push_str(&line[..index]);
                        break;
                    } else {
                        panic!("File is not in correct format");
                    }
                }
                None => content.push_str(&line),
            }
        }

        let mut reply = false;
        let mut retweet = false;

        for line in lines {
            if line == "reply" {
                reply = true;
            } else if line == "retweet" {
                retweet = true;
            } else {
                panic!("File is not in correct format");
            }
        }

        Ok(TweetReader {
            username,
            content,
            reply,
            retweet,
        })
    }
}

impl Summary for TweetReader {
    fn msg_len(&self) -> usize {
        self.content.len()
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

    fn get_info(&self) -> String {
        if self.reply && self.retweet {
            format!("Tweet from @{} (reply, retweet)", self.username)
        } else if self.reply {
            format!("Tweet from @{} (reply)", self.username)
        } else if self.retweet {
            format!("Tweet from @{} (retweet)", self.username)
        } else {
            format!("Tweet from @{}", self.username)
        }
    }
}
