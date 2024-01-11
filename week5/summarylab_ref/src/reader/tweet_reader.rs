use crate::{Reader, Summary};
use std::io;

/// A struct that represents a tweet.
#[derive(Debug)]
pub struct TweetReader {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Reader for TweetReader {
    /// Creates a new [`TweetReader`] from a `String` of data.
    ///
    /// This method takes in a `file_str`, which is a `String` containing the same data
    /// as a file.
    /// The file / `String` should be in the following format:
    ///
    /// ```text
    /// @{username}
    /// "{content}"
    /// {?"reply"}
    /// {?"retweet"}
    /// ```
    ///
    /// Note that content can span multiple lines, and will always be surrounded by double quotes.
    ///
    /// If the tweet is a reply, the word `"reply"` will be on a new line after the content,
    /// and if the tweet is also a retweet, the word `"retweet"`
    /// will be on a new line after content (and after `"reply"` if it is there too).
    ///
    /// You're going to want to do the exact same thing as you did for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader) with how you handle errors.
    /// Refer to the documentation for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader)'s version of
    /// [`parse`](crate::reader::email_reader::EmailReader::parse).
    fn parse(file_str: String) -> Result<TweetReader, std::io::Error> {
        let mut lines = file_str.lines();

        let username = lines.next().unwrap()[1..].to_string();

        let mut content = lines.next().unwrap().to_string();
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
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "File is not in the correct format",
                        ));
                    }
                }
                None => content.push_str(line),
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
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
