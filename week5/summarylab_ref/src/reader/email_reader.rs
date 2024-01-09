use crate::Summary;
use std::io;

/// A struct that represents an email message.
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
    /// {message}
    /// To: {receiver}
    /// ```
    ///
    /// If no message is found, assume the message is `""`.
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
        let mut message = String::from("");
        let mut to = String::new();
        let mut subject = String::new();

        // Iterate over each line in the file
        for (num, line) in file_str.lines().enumerate() {
            let line = line.to_string();

            let components = line.split(": ").collect::<Vec<&str>>();

            if num == 2 && components.len() == 1 {
                message = line;
                continue;
            }

            if components.len() != 2 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File is not in the correct format",
                ));
            }

            let title = components[0];
            let content = components[1];

            match title {
                "Subject" => subject = content.to_string(),
                "From" => from = content.to_string(),
                "To" => to = content.to_string(),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "File is not in the correct format",
                    ))
                }
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
    fn msg_len(&self) -> usize {
        self.message.len()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.from, self.message)
    }

    fn get_info(&self) -> String {
        format!("{}\nFrom: {}, To {}", self.subject, self.from, self.to)
    }
}
