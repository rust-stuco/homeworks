/// This module contains the [`EmailReader`](email_reader::EmailReader) type,
/// as well as its method implementations.
///
/// The [`EmailReader`](email_reader::EmailReader) struct will model an email message.
/// [`EmailReader`](email_reader::EmailReader) should have the attributes similar to these:
///  - `subject: String`
///  - `sender: String`
///  - `receiver: String`
///  - `message: String`
///
/// All of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// Once you've added the fields to the struct, implement the [`Reader`](crate::Reader) trait:
/// - [`parse`](crate::Reader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`EmailReader`](email_reader::EmailReader)
/// based on that file. See the documentation on [`EmailReader`](email_reader::EmailReader)
/// for more details about how the files should be formatted
///
/// We also want [`EmailReader`](email_reader::EmailReader)s to be able to summarize themselves.
/// We'll want to implement the [`Summary`](crate::Summary) trait:
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the message
/// - [`summarize`](crate::Summary::summarize) should return a string
/// that contains `"{sender}: {message}"`, but with a maximum of 280 characters.
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"{subject}\nFrom: {sender}, To: {receiver}"`
///
/// See the type documentation for [`EmailReader`](email_reader::EmailReader) for more details.
pub mod email_reader;

/// This module contains the [`TweetReader`](tweet_reader::TweetReader) type,
/// as well as its method implementations.
///
/// The [`TweetReader`](tweet_reader::TweetReader) struct will model a tweet.
/// [`TweetReader`](tweet_reader::TweetReader) should have the following attributes:
/// - `username: String`
/// - `content: String`
/// - `reply: bool`
/// - `retweet: bool`
///
/// All of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// We'll want to implement both [`Reader`](crate::Reader) and [`Summary`](crate::Summary)
/// for this type as well.
///
/// - [`parse`](crate::Reader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`TweetReader`](tweet_reader::TweetReader)
/// based on that file
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the content
/// - [`summarize`](crate::Summary::summarize) should return a string that
/// contains `"@<username>: <content>"`
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"Tweet from @<username> "`. with (reply) or (retweet) appended
///
/// See the type documentation for [`TweetReader`](tweet_reader::TweetReader) for more details.
pub mod tweet_reader;

#[cfg(test)]
pub mod tests;
