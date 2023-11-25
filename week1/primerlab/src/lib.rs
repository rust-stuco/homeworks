//! ### 98-008: Intro to the Rust Programming Language
//!
//! # Primer Lab
//!
//! The goal of this homework is to make sure that you understand Rust's basic syntax.
//!
//! We recommend you make use of the [Rust Book](https://!doc.rust-lang.org/book/title-page.html)
//! chapters 1-3, as the "textbook" of this course. It's generally easy to follow along with,
//! and would make handy reference material for the homeworks in this course.
//!
//! We've tried to calibrate this homework to take around an hour,
//! so if you are spending much more than that, please let us know!
//! This time limit does not include setting up Rust on your machine though,
//! so if you have trouble with installing Rust, make sure to ask for help.
//!
//! # Setup
//!
//! Make sure you have Rust installed. You should have installed Rust through `rustup`.
//! To confirm, run:
//!
//! ```sh
//! $ rustup --version
//! rustup 1.26.0 (2023-11-14)
//! $ cargo --version
//! cargo 1.74.0 (ecb9851af 2023-10-18)
//! ```
//!
//! Make sure both of those commands execute successfully, and have relatively recent dates.
//! If the dates are not recent, you can update `rustup` with `rustup update`.
//!
//! Once you have `cargo` installed, run `cargo doc` in this directory to generate documentation
//! for this homework.
//!
//! ```sh
//! $ cargo doc
//! Documenting primerlab v0.1.0 (<path>/primerlab)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.11s
//! Generated <path>/primerlab/target/doc/primerlab/index.html
//! ```
//!
//! `cargo doc` will generate an HTML file that you can view in your browser!
//! It might be easier to read the markdown-rendered docs than the comments in the handout.
//!
//! # Part 1: Exercises
//!
//! In this part, you will look at 8 files that do not compile,
//! and modify them each in some way to make them compile.
//!
//! Run this command in the root of your project (parent directory of the `src/` folder):
//! ```sh
//! $ cargo test -- fixme
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

pub mod exercises;
pub mod functions;
