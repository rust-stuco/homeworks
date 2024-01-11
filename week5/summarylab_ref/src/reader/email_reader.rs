use crate::Summary;
use std::io;

/// A struct that represents an email message.
#[derive(Debug)]
pub struct EmailReader {
    subject: String,
    from: String,
    to: String,
    message: String,
}

impl EmailReader {
    /// Creates a new [`EmailReader`] given a path to a file.
    ///
    /// Internally, reads a file to a [`String`],
    /// and then calls [`EmailReader::parse`] on that [`String`].
    pub fn new(file_path: String) -> Result<EmailReader, io::Error> {
        let file_str = crate::read_file(file_path)?;
        Self::parse(file_str)
    }

    /// Creates a new [`EmailReader`] from a `String` of data.
    ///
    /// This method takes in a `file_str`, which is a `String` containing the same data
    /// as a file.
    ///
    /// The file should be in the following format:
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
    ///
    /// ---
    ///
    /// This function returns a [`Result<EmailReader, std::io::Error>`].
    /// Here are the cases we need to handle when reading and parsing data from a file:
    /// - The file does not exist or we cannot read it
    /// - The file is not in the correct format
    ///
    /// We've implemented [`EmailReader::new`] for you to deal with opening a file and
    /// returning a [`std::io::Error`] if something goes wrong.
    /// Also, take a look at [`read_file`](crate::read_file) and try to understand what is
    /// going on there too. At a minimum, you should understand what the `?` operator is doing.
    ///
    /// We can leverage the standard library to handle opening and reading files, and it will
    /// tell us if something goes wrong with the IO.
    /// The standard library does not, however, know if the file contains the correct format
    /// of an email we are expecting.
    /// So you will have to create your own error with an error kind
    /// [`InvalidData`](std::io::ErrorKind::InvalidData) and the message
    /// `"File is not in the correct format"`.
    /// See [`std::io::Error::new`] for more details on how to do this.
    pub fn parse(file_str: String) -> Result<EmailReader, io::Error> {
        let mut from = String::new();
        let mut to = String::new();
        let mut subject = String::new();

        let lines: Vec<&str> = file_str.lines().collect();

        if lines.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File is not in the correct format",
            ));
        }

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
            let from_line: Vec<&str> = lines[1].split(": ").collect();
            if from_line.len() != 2 || from_line[0] != "From" || from_line[1].is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }
            from.push_str(from_line[1]);
        }

        {
            let to_line: Vec<&str> = lines[2].split(": ").collect();
            if to_line.len() != 2 || to_line[0] != "To" || to_line[1].is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }
            to.push_str(to_line[1]);
        }

        if lines.len() == 3 {
            return Ok(EmailReader {
                subject,
                from,
                to,
                message: "".to_string(),
            });
        }

        let message = lines[3..].join("\n");

        Ok(EmailReader {
            subject,
            from,
            to,
            message,
        })
    }
}

impl Summary for EmailReader {
    fn msg_len(&self) -> usize {
        self.message.len()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.from, self.message)
    }

    fn get_info(&self) -> String {
        format!("{}\nFrom: {}, To: {}", self.subject, self.from, self.to)
    }
}
