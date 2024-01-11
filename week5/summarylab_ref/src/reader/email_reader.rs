use crate::{Reader, Summary};
use std::io;

/// A struct that represents an email message.
///
/// The file / [`String`] should be in the following format:
///
/// ```text
/// Subject: {subject}
/// From: {sender}
/// To: {receiver}
/// {message}
/// ```
///
/// The message can span multiple lines. If no message is found, assume the message is `""`.
/// The subject, sender, and receiver, however, cannot be empty strings.
#[derive(Debug)]
pub struct EmailReader {
    subject: String,
    sender: String,
    receiver: String,
    message: String,
}

impl Reader for EmailReader {
    /// Creates a new [`EmailReader`] from a [`String`] of data.
    ///
    /// This method takes in a `file_str`, which is a [`String`] containing the same data
    /// as a file. If the string is not in the correct format, returns an IO error
    /// with the message `"File is not in the correct format"`.
    ///
    /// ---
    ///
    /// This function returns a [`Result<EmailReader, std::io::Error>`].
    /// Here are the cases we need to handle when reading and parsing data from a file:
    /// - The file does not exist or we cannot read it
    /// - The file is not in the correct format
    ///
    /// We've implemented [`Reader::new`] for you to deal with opening a file and
    /// returning a [`std::io::Error`] if something goes wrong. Take a look at the
    /// default implementation and make sure you understand what it is doing.
    /// At a minimum, you should understand what the `?` operator is there for.
    ///
    /// So we are able to leverage the standard library to handle opening and reading files,
    /// and it will tell us if something goes wrong with the IO when we call `new`.
    /// The standard library does not, however, know if the file contains the correct format
    /// of an email we are expecting.
    ///
    /// You will have to create your own error with an error kind
    /// [`InvalidData`](std::io::ErrorKind::InvalidData) and the message
    /// `"File is not in the correct format"`.
    /// See [`std::io::Error::new`] for more details on how to do this.
    fn parse(file_str: String) -> Result<EmailReader, io::Error> {
        let lines: Vec<&str> = file_str.lines().collect();

        if lines.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File is not in the correct format",
            ));
        }

        let mut subject = String::new();
        let mut sender = String::new();
        let mut receiver = String::new();

        {
            let subject_line: Vec<&str> = lines[0].split(": ").collect();
            if subject_line.len() != 2 || subject_line[0] != "Subject" || subject_line[1].is_empty()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }
            subject.push_str(subject_line[1]);
        }

        {
            let sender_line: Vec<&str> = lines[1].split(": ").collect();
            if sender_line.len() != 2 || sender_line[0] != "From" || sender_line[1].is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }
            sender.push_str(sender_line[1]);
        }

        {
            let receiver_line: Vec<&str> = lines[2].split(": ").collect();
            if receiver_line.len() != 2 || receiver_line[0] != "To" || receiver_line[1].is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }
            receiver.push_str(receiver_line[1]);
        }

        if lines.len() == 3 {
            return Ok(EmailReader {
                subject,
                sender,
                receiver,
                message: "".to_string(),
            });
        }

        let message = lines[3..].join("\n");

        Ok(EmailReader {
            subject,
            sender,
            receiver,
            message,
        })
    }
}

impl Summary for EmailReader {
    /// Returns the length of the message in the email (not the full email).
    fn msg_len(&self) -> usize {
        self.message.len()
    }

    /// Returns a [`String`] that is equivalent to `"{sender}: {message}"`.
    fn summarize(&self) -> String {
        format!("{}: {}", self.sender, self.message)
    }

    /// Returns a [`String`] that is equivalent to "{subject}\nFrom: {sender}, To: {receiver}"`.
    fn get_info(&self) -> String {
        format!("{}\nFrom: {}, To: {}", self.subject, self.sender, self.receiver)
    }
}
