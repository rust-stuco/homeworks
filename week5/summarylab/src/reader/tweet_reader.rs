#![allow(unused_variables, unused_imports)]

use crate::{Reader, Summary};
use std::io;

/// A struct that represents a tweet.
///
/// The file / [`String`] should be in the following format:
///
/// ```text
/// @{username}
/// "{content}"
/// {"" | "reply" | "retweet"}
/// ```
///
/// Note that content can span multiple lines, and will always be surrounded by double quotes.
/// If there is no content, there will just be a line containing `""`.
///
/// If the tweet is a reply, the word `"reply"` will be on a new line after the content,
/// and if instead the tweet is a retweet, the word `"retweet"`
/// will be on a new line after the content.
///
/// For this assignment, a Tweet can only be one of a normal tweet, a reply, or a retweet.
#[derive(Debug)]
pub struct TweetReader {
    _replace_me: (),
}

impl Reader for TweetReader {
    /// Creates a new [`TweetReader`] from a [`String`] of data.
    ///
    /// This method takes in a `file_str`, which is a [`String`] containing the same data
    /// as a file.
    ///
    /// You're going to want to do the exact same thing as you did for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader) with how you handle errors.
    /// Refer to the documentation for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader)'s version of
    /// [`parse`](crate::reader::email_reader::EmailReader::parse).
    fn parse(file_str: String) -> Result<TweetReader, std::io::Error> {
        // lines is a vector of all the newline-separated lines as &str from the file_str
        let lines: Vec<&str> = file_str.lines().collect();

        todo!()
    }
}

impl Summary for TweetReader {
    /// Returns the length of the content in the tweet (not the full tweet).
    fn msg_len(&self) -> usize {
        todo!()
    }

    /// Returns a string equivalent to `"@{username}: {content}"`. No maximum length.
    fn summarize(&self) -> String {
        todo!()
    }

    /// Returns a string equivalent to
    /// `"Tweet from @{username}"`. with (reply) or (retweet) appended if applicable.
    ///
    /// For example, if `@ferris` made a retweet,
    /// this would return `"Tweet from @ferris (retweet)"`.
    fn get_info(&self) -> String {
        todo!()
    }
}
