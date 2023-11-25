//! ### 98-008: Intro to the Rust Programming Language
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
//! feel free to reach out for help!
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
//! # Part 1: Move Semantics
//!
//! Like the previous homework, you will modify 6 files that don't pass the borrow checker
//! by changing a few things to make them compile!
//!
//! Run this command in the root of your project (parent directory of the `src/` folder):
//! ```sh
//! $ cargo test -- move_semantics
//! ```
//! **What does the error say?** Go into `src/exercises/fixme1.rs` and make the change.
//!
//! When you are able to run `cargo test -- fixme` without any errors,
//! move on to the next exercise by uncommenting `mod fixme2` in `src/exercises/mod.rs`.
//! Go through all 8 of the exercises and
//! make sure that the 8 test cases pass when you run `cargo test -- fixme`.
//!
//!
//! # Part 2: Function implementations
//!
//! For this part, you will need to implement 4 simple functions in Rust.
//! In `src/lib.rs`, you will find 4 functions with a `todo!()` inside of them.
//! Replace that `todo!()` with your implementation, according to the comment specification.
//! One of the functions requires you to implement it in a certain way,
//! so make sure to read those comments carefully.
//!
//! To test all of your functions, run:
//! ```sh
//! $ cargo test
//! ```
//! If you find that a test is running slowly, try running `cargo test --release`,
//! which runs the tests in release mode. We'll talk about this more in future weeks,
//! but basically it runs the tests with more compiler optimizations (like the `-O2` flag).
//! Also, if you want to run a specific test, run `cargo test -- it_works`
//! with the name of the test instead of `it_works`.
//!
//! # Submission
//! - Run `make handin`
//! - Submit the resultant `handin.tar` to Gradescope.
//!
//! If you do not have `make` and `tar` installed on your system,
//! install `make` and `tar` on your machine or use the CMU Linux SSH machines.
//! If you need help with this, please reach out to us!
//!
//! ## Collaboration
//! In general, feel free to discuss homeworks with other students!
//! As long as you do not copy someone else's work, any communication is fair game.
//! Feel free to discuss on the course Discord
//! so that other students can see your questions and answers, as well.
//!
//! ## Feedback
//! We would also like to reiterate that you should let us know if you spent
//! anywhere in significant excess of an hour on this homework.
//! These assignments are being deployed for the first time,
//! and we are definitely open to feedback regarding the length and difficulty.
//! In addition, Rust has a notoriously steep learning curve,
//! so if you find yourself not understanding the concepts,
//! you should reach out to us and let us know as well ---
//! chances are, you're not the only one!

mod move_semantics;
mod strings;