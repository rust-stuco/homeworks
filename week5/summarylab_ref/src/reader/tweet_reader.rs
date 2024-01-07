use crate::summarize::Summary;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// TweetReader is a struct that represents a tweet.
/// It implements the Summary trait as well as the following methods:
/// `new(file_path: String) -> Result<TweetReader, std::io::Error>` - creates a new TweetReader from file
struct TweetReader {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl TweetReader {
    /// Creates a new TweetReader
    /// file_path - is the path to the text message that will be read
    /// The file should be in the following format:
    /// @<username>
    /// "<content>"
    ///
    /// Content can span multiple lines. Content will be surrounded by quotes.
    /// If the tweet is a reply, the word "reply" will be on a new line after content
    /// If the tweet is a retweet, the word "retweet" will be on a new line after content or reply
    ///
    /// Use the use std::fs and std::io to read the file
    /// and get the sender, receiver, subject and message
    ///
    /// If the file does not exist, panic with the message "File not found"
    /// If the file is not in the correct format, panic with the message "File is not in correct format"
    /// If no message is found, assume the message is ""
    fn new(file_path: String) -> Result<TweetReader, std::io::Error> {
        let file = File::open(file_path).expect("File not found");
        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(|line| line.unwrap());

        let username = lines.next().unwrap().to_string();

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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
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

    fn msg_len(&self) -> usize {
        self.content.len()
    }
}
