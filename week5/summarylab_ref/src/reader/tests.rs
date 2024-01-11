#[cfg(test)]
mod email_tests {
    use crate::reader::email_reader::EmailReader;
    use crate::{Reader, Summary};

    #[test]
    fn test_basic_email() {
        let s = "Subject: Party Time
From: Jake
To: David
Just finished my work. Time for a break!";
        let result = EmailReader::parse(s.to_string()).unwrap();

        assert_eq!(result.msg_len(), 40);
        assert_eq!(
            result.summarize(),
            "Jake: Just finished my work. Time for a break!"
        );
        assert_eq!(result.get_info(), "Party Time\nFrom: Jake, To: David");
    }

    #[test]
    fn test_empty_email() {
        let s = "";
        let result = EmailReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_missing_fields() {
        let s = "Subject: Test Email
To: Recipient";
        let result = EmailReader::parse(s.to_string());
        println!("{:?}", result);
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
        assert_eq!(
            result.as_ref().unwrap_err().to_string(),
            "File is not in the correct format"
        );
    }

    #[test]
    fn test_summarize_short_email() {
        let s = "Subject: Party Time
From: Jake
To: David
Just finished my work. Time for a break!";
        let reader = EmailReader::parse(s.to_string()).unwrap();
        let summary = reader.summarize();
        assert_eq!(summary, "Jake: Just finished my work. Time for a break!");
    }

    #[test]
    fn test_summarize_formal_email() {
        let s = "Subject: Meeting Reschedule
From: Executive Team
To: Department Heads
Please note the change in meeting schedule...";
        let reader = EmailReader::parse(s.to_string()).unwrap();
        let summary = reader.summarize();
        assert!(summary.starts_with("Executive Team: "));
        assert!(reader.get_info().contains("Meeting Reschedule"));
    }

    #[test]
    fn test_empty_subject() {
        let email_str = "Subject: \nFrom: sender\nTo: recipient\n";
        let result = EmailReader::parse(email_str.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_empty_sender() {
        let email_str = "Subject: Test\nFrom: \nTo: recipient\n";
        let result = EmailReader::parse(email_str.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_empty_recipient() {
        let email_str = "Subject: Test\nFrom: sender\nTo: ";
        let result = EmailReader::parse(email_str.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_summarize_extremely_long_message() {
        let subject = "Long Message Subject";
        let sender = "LongSenderName@example.com";
        let recipient = "LongRecipientName@example.org";

        // Generate a very long message, exceeding typical lengths
        let mut message = String::new();
        for _ in 0..1000 {
            message.push_str("This is a sentence in the very long message. ");
        }

        let email_str = format!(
            "Subject: {}\nFrom: {}\nTo: {}\n{}",
            subject, sender, recipient, message
        );

        let reader = EmailReader::parse(email_str.to_string()).unwrap();
        let summary = reader.summarize();
        assert_eq!(summary.len(), 280);

        assert!(summary.contains(sender));
        assert!(summary.contains("This is a sentence in the very long message. "));
    }
}
