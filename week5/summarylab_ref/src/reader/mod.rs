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
/// Once you've added the fields to the struct, implement the [`Reader`](crate::Reader) trait
/// that has been defined for you in `src/lib.rs`:
/// - [`parse`](crate::Reader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`EmailReader`](email_reader::EmailReader)
/// based on that file (see the documentation on [`EmailReader`](email_reader::EmailReader)
/// for more details about how the files should be formatted)
///
/// We also want [`EmailReader`](email_reader::EmailReader)s to be able to summarize themselves.
/// We'll want to implement the [`Summary`](crate::Summary) trait, which is also defined
/// in `src/lib.rs`:
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the message
/// - [`summarize`](crate::Summary::summarize) should return a string
/// that contains `"{sender}: {message}"`, but with a maximum of 280 characters
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"{subject}\nFrom: {sender}, To: {receiver}"`
///
/// See the type documentation for [`EmailReader`](email_reader::EmailReader)
/// by clicking the hyperlink for more details.
pub mod email_reader;

/// This module contains the [`TweetReader`](tweet_reader::TweetReader) type,
/// as well as its method implementations.
///
/// The [`TweetReader`](tweet_reader::TweetReader) struct will model a tweet.
/// [`TweetReader`](tweet_reader::TweetReader) should have _at minimum_ the following attributes:
/// - `username: String`
/// - `content: String`
///
/// **A Tweet can also either be a retweet or a reply, on top of being a normal tweet.**
///
/// It is up to you how you want to represent this type of state. You could have two booleans
/// that tell you if it is a retweet or a reply, but make sure that they aren't both true.
/// If only there was a way to _enumerate_ the types of states we could be in...
///
/// _If you choose to create some new type, make sure to annotate it with `#[derive(Debug)]`_
/// _so you can easily debug it later._
///
/// Again, all of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// We'll want to implement both [`Reader`](crate::Reader) and [`Summary`](crate::Summary)
/// for this type as well, both located in `src/lib.rs`.
///
/// - [`parse`](crate::Reader::parse): This method will take in a [`String`] that was
/// read from a file and create a new [`TweetReader`](tweet_reader::TweetReader)
/// based on that file
///
/// - [`msg_len`](crate::Summary::msg_len) should return the length of the content
/// - [`summarize`](crate::Summary::summarize) should return a string that
/// contains `"@{username}: {content}"`
/// - [`get_info`](crate::Summary::get_info) should return a string that contains
/// `"Tweet from @{username}"` with `"(reply)"` or `"(retweet)"` appended if applicable
///
/// See the type documentation for [`TweetReader`](tweet_reader::TweetReader)
/// by clicking the hyperlink for more details.
pub mod tweet_reader;

#[cfg(test)]
pub mod tests;
