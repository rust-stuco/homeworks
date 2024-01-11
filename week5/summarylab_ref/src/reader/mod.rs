/// This module contains the [`EmailReader`](email_reader::EmailReader) type,
/// as well as its method implementations.
///
/// The [`EmailReader`](email_reader::EmailReader) struct will model an email message.
/// [`EmailReader`](email_reader::EmailReader) should have the following attributes:
///  - `subject: String`
///  - `from: String`
///  - `to: String`
///  - `message: String`
///
/// All of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// Once you've added the fields to the struct, implement the following method:
/// - [`parse`](email_reader::EmailReader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`EmailReader`](email_reader::EmailReader)
/// based on that file.
///
/// We also want [`EmailReader`](email_reader::EmailReader)s to be able to summarize themselves.
/// We'll want to implement the following methods
/// [`msg_len`](crate::Summary::msg_len), [`summarize`](crate::Summary::summarize),
/// and [`get_info`](crate::Summary::get_info),
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the message
/// - [`summarize`](crate::Summary::summarize) should return a string
/// that contains `"<sender>: message"`
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"{subject}\nFrom: {sender}, To: {receiver}"`
///
/// Since we'll probably need to do this again 😉, instead of implementing all three methods
/// in the main `impl` block, we'll instead implement the trait [`Summary`](crate::Summary)
/// that already has these 3 methods defined for us!
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
/// Once you've added the fields to the struct, implement the following method:
/// - [`parse`](tweet_reader::TweetReader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`TweetReader`](tweet_reader::TweetReader)
/// based on that file.
///
/// We also want [`TweetReader`](tweet_reader::TweetReader)s to be able to summarize themselves.
/// We'll implement the following methods
/// [`msg_len`](crate::Summary::msg_len), [`summarize`](crate::Summary::summarize),
/// and [`get_info`](crate::Summary::get_info),
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the message.
/// - [`summarize`](crate::Summary::summarize) should return a string that
/// contains `"@<username>: <content>"`.
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"Tweet from @<username> "`. with (reply) or (retweet) appended.
///
/// See the type documentation for [`TweetReader`](tweet_reader::TweetReader) for more details.
pub mod tweet_reader;

#[cfg(test)]
pub mod tests;
