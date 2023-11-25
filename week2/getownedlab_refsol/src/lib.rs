//! ### 98-008: Intro to the Rust Programming Language
//!
//!
//!
//! # Get Owned Lab
//!
//! The goal of this homework is to familiarize yourself with Rust's system of Ownership.
//!
//! Lecture was heavily based on Chapter 4 of the
//! [Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
//! We **_strongly_** suggest that you make sure you understand the rules of Ownership
//! before attempting this homework, and this text is a great place to reference!
//!
//! This homework is a bit shorter than the previous one, mainly so you can have extra time to
//! read through the book and stew with the concepts a bit.
//!
//! If you find yourself getting stuck,
//! it might be good to take a small break and just go over the borrow checker's rules.
//!
//! Remember to always read the error message that the compiler gives you, and as always,
//! we encourage you to reach out for help!
//!
//!
//!
//! # Setup
//!
//! Run `cargo doc` at the root of this package to generate the documentation for this lab.
//!
//! ```sh
//! $ cargo doc
//! Documenting getownedlab v0.1.0 (<path>/getownedlab)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.11s
//! Generated <path>/getownedlab/target/doc/getownedlab/index.html
//! ```
//!
//! `cargo doc` will generate an HTML file that you can view in your browser!
//! It might be easier to read the markdown-rendered docs than the comments in the handout.
//!
//!
//!
//! # Part 1: Move Semantics
//!
//! Like the previous homework, you will modify 5 files that don't pass the borrow checker
//! by changing a few things to make them compile! The first 3 exercises involve the `Vec` type,
//! which is an _owned_ type.
//!
//! For this homework, all you need to know is the `vec![]` macro
//! and the `push` method on vectors. Recall the `vec![]` creates a new vector with elements, and
//! `push` appends an element onto the end of a vector.
//!
//! Run this command in the root of your project (parent directory of the `src/` folder):
//! ```sh
//! $ cargo test
//! ```
//! **What does the error say?** Go into `src/move_semantics/move_semantics1.rs`
//! and make the change.
//!
//! When you are able to run `cargo test` without any errors, move on to the next
//! exercise by uncommenting `pub mod move_semantics2` in `src/move_semantics/mod.rs`.
//! Go through all 5 of the exercises in `src/move_semantics` and
//! make sure that all the test cases pass when you run `cargo test`.
//!
//!
//!
//! # Part 2: Strings
//!
//! In this section, you'll be working with `String` and `&str` instead of `Vec`.
//!
//!
//! The first three exercises in `src/strings` are just like the previous section,
//! where you need to make a change to pass the borrow checker and compile.
//! Uncomment `pub mod strings1;` in `src/strings/mod.rs` and run `cargo test`.
//! What does the compiler tell you? Make the change in `src/strings/strings1.rs` and then
//! move onto the next exercises by uncommenting the other `pub mod strings_;`.
//!
//! A useful thing to remember here is that a `&String` can be _coerced_ into a `&str`.
//! Other than that, just remember to make sure that you always read what the compiler tells you
//! (it may or may not give away the answer)!
//!
//! For `strings4.rs`, you will need to look at documentation. You _could_ just google the answer,
//! but it would be good to familiarize yourself with Rust documentation.
//! Here is the official documentation for
//! [`str`](https://doc.rust-lang.org/std/primitive.str.html).
//!
//! There's a search bar at the top of the documentation that you can also make use of.
//! Try searching for methods called `trim` there!
//!
//!
//!
//! # Submission
//! - Run `make handin`
//! - Submit the resultant `handin.tar` to Gradescope.
//!
//! If you do not have `make` and `tar` installed on your system,
//! install `make` and `tar` on your machine or use the CMU Linux SSH machines.
//! If you need help with this, please reach out to us!
//!
//!
//!
//! # Collaboration
//! In general, feel free to discuss homeworks with other students!
//! As long as you do not copy someone else's work, any communication is fair game.
//! Feel free to discuss on the course Discord
//! so that other students can see your questions and answers, as well.
//!
//!
//!
//! # Feedback
//! We would like to reiterate that you should let us know if you spent
//! anywhere in significant excess of an hour on this homework.
//! These assignments are being deployed for the first time,
//! and we are definitely open to feedback regarding the length and difficulty.
//! In addition, Rust has a notoriously steep learning curve,
//! so if you find yourself not understanding the concepts,
//! you should reach out to us and let us know as well ---
//! chances are, you're not the only one!

pub mod move_semantics;
pub mod strings;