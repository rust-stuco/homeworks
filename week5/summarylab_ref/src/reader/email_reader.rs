use crate::Summary;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// EmailReader is a struct that represents an email message.
/// It implements the Summary trait as well as the following methods:
/// `new(file_path: String) -> Result<EmailReader, std::io::Error>` - creates a new EmailReader from file
pub struct EmailReader {
    subject: String,
    from: String,
    to: String,
    message: String,
}

impl EmailReader {
    /// Creates a new EmailReader
    /// file_path - is the path to the text message that will be read
    /// The file should be in the following format:
    /// Subject: <subject>
    /// From: <sender>
    /// <message>
    /// To: <receiver>
    /// Use the std::fs and std::io libraries to read the file
    /// and get the sender, receiver, subject and message
    ///
    /// If the file does not exist, panic with the message "File not found"
    /// If the file is not in the correct format, panic with the message "File is not in correct format"
    /// If no message is found, assume the message is ""
    pub fn new(file_path: String) -> Result<EmailReader, std::io::Error> {
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
