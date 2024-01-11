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
    fn test_summarize_short() {
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

#[cfg(test)]
mod tweet_tests {
    use crate::reader::tweet_reader::TweetReader;
    use crate::{Reader, Summary};

    #[test]
    fn test_basic_tweet() {
        let s = "@1CoolDavid
\"Hey Rustaceans!
Excited to dive into some advanced topics today. Let's discuss ownership and lifetimes.
#RustLang\"";
        let result = TweetReader::parse(s.to_string()).unwrap();
        assert_eq!(result.msg_len(), 113);
        assert_eq!(
            result.summarize(),
            "@1CoolDavid: Hey Rustaceans!
Excited to dive into some advanced topics today. Let's discuss ownership and lifetimes.
#RustLang"
        );
        assert_eq!(result.get_info(), "Tweet from @1CoolDavid");
    }

    #[test]
    fn test_empty_tweet() {
        let s = "";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_missing_fields() {
        let s = "@Connor";
        let result = TweetReader::parse(s.to_string());
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
    fn test_summarize_retweet() {
        let s = "@FerrisTheCrab
\"@1CoolDavid Absolutely, David! Ownership is key to Rust's safety.
Lifetimes help us manage references. Students, any questions so far?
#RustLearning\"
retweet";
        let reader = TweetReader::parse(s.to_string()).unwrap();
        assert_eq!(reader.msg_len(), 148);
        let summary = reader.summarize();
        assert!(summary.starts_with("@FerrisTheCrab: @1CoolDavid Absolutely,"));
        assert_eq!(reader.get_info(), "Tweet from @FerrisTheCrab (retweet)");
    }

    #[test]
    fn test_missing_username() {
        let s = "\"Hello world!\"";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_invalid_username_format() {
        let s = "InvalidUsername
\"Invalid tweet\"";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_missing_initial_quote() {
        let s = "@user
This is content without a starting quote";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_missing_ending_quote() {
        let s = "@user
\"This content is missing an ending quote";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_invalid_tweet_type() {
        let s = "@user
\"Content\"
invalid";
        let result = TweetReader::parse(s.to_string());
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_msg_len_with_empty_content() {
        let s = "@user
\"\"";
        let reader = TweetReader::parse(s.to_string()).unwrap();
        assert_eq!(reader.msg_len(), 0);
    }

    #[test]
    fn test_get_info_reply() {
        let tweet_str = "@user
\"Content\"
reply";
        let reader = TweetReader::parse(tweet_str.to_string()).unwrap();
        assert_eq!(reader.get_info(), "Tweet from @user (reply)");
    }

    #[test]
    fn test_get_info_retweet_with_long_username() {
        let username = "VeryLongUserName12345";
        let tweet_str = format!(
            "@{username}
\"Content\"
retweet"
        );
        let reader = TweetReader::parse(tweet_str.to_string()).unwrap();
        assert_eq!(
            reader.get_info(),
            "Tweet from @VeryLongUserName12345 (retweet)"
        );
    }

    #[test]
    fn test_get_info_reply_with_special_characters_in_username() {
        let username = "user_with-underscores.123";
        let tweet_str = format!(
            "@{username}
\"Content\"
reply"
        );
        let reader = TweetReader::parse(tweet_str.to_string()).unwrap();
        assert_eq!(
            reader.get_info(),
            "Tweet from @user_with-underscores.123 (reply)"
        );
    }

    #[test]
    fn test_get_info_empty_content() {
        let tweet_str = "@user
\"\"
retweet";
        let reader = TweetReader::parse(tweet_str.to_string()).unwrap();
        assert_eq!(reader.get_info(), "Tweet from @user (retweet)");
    }

    #[test]
    fn test_summarize_extremely_long_content() {
        let username = "LongUserName";
        let mut content = String::new();
        for _ in 0..1000 {
            content.push_str("This is a sentence in the very long tweet. ");
        }

        let tweet_str = format!(
            "@{}
\"{}\"",
            username, content
        );
        let reader = TweetReader::parse(tweet_str.to_string()).unwrap();

        assert!(reader
            .summarize()
            .starts_with("@LongUserName: This is a sentence in the very long tweet."));
        assert!(reader.summarize().contains(username));
        assert!(reader.summarize().contains(&content));
    }
}
