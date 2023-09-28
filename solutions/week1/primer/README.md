# Intro to Rust (98-008): HW 1

The goal of this homework is to make sure that you understand Rust's basic syntax!

We _**strongly**_ recommend you look at the [Rust Book](https://doc.rust-lang.org/book/title-page.html), chapters 1-3.
It's not very long and also quite easy to read. Since we are following this book in lecture, it serves as the "textbook" of this course. **You do not have to read all of it**: it should really just be a reference that you can look to for simple questions.

There are 2 parts to this homework: several exercises and then several function implementations.
We expect you to finish this homework in _a little over 30 minutes_, so if you are spending more than that, please let us know! This time limit does not include setting up Rust on your machine though, so if you have trouble with installing Rust, make sure to ask for help.

## Setup

Make sure you have Rust installed. You should have installed Rust through `rustup`, so just to check run this:
```sh
$ rustup --version
$ cargo --version
```
Make sure both of those commands execute successfully, and make sure the dates they give back are relatively recent.

If they are not recent, run this (or run it regardless, it's good to have fully updated stuff):
```sh
$ rustup update
```

To get started, run this command in the directory of your choosing:
```sh
$ cargo new primer
```
This will create a new `cargo` project that you will be modifying.
Replace the `primer/src` directory with the `src/` folder we have provided.
TODO


## Part 1: Exercises

In this part, you will look at 8 files that do not compile, and modify them each in some way to make them compile. **Each of these exercises should take around 30 seconds each** (if you were paying attention in lecture).

Going into `src/exercises/mod.rs`, you'll see a bunch of commented `mod fixme_` lines.
Run this command in the root of your project (parent directory of the `src/` folder):
```sh
$ cargo test -- fixme
```
**What does the error say?** Go into `src/exercises/fixme1.rs` and make the change.

When you are able to run `cargo test -- fixme` without any errors, move on to the next exercise by uncommenting `mod fixme2`. Go through all 8 of the exercises and make sure that the 8 test cases pass when you run `cargo test -- fixme`.

Try to aim for 10 minutes to complete all of them. Note that for this part, you should only be modifying (and submitting) files named `fixme_.rs`.


## Part 2: Function implementations

You will then need to implement 4 basic functions (that you've likely seen before) in Rust.
Go into `src/lib.rs`, and you will find 4 functions with a `todo!()` inside of them. Replace that `todo!()` with your implementation of what the comment describes the function to be! 2 of the 4 functions require you to implement them a certain way, so make sure to read those comments carefully.

To test _all_ of your functions, run this:
```sh
$ cargo test
```
If you find that a test is running slowly, try running `cargo test --release`, whcih runs the tests in release mode. We'll talk about this more in future weeks, but basically it runs the tests with more compiler optimizations (like the `-O2` flag). Also, if you want to run a specific test, run `cargo test -- it_works` with the name of the test instead of `it_works`.

Finally, go into `main.rs` and replace the `todo!()` with something that will tell the user if their input is a prime number or not (this is not a trick question). You can run this program by running:
```sh
$ cargo run
```


## Collaboration
In general, feel free to ask us or other students questions! As long as you do not copy someone else's work, any communication is fair game. Though please try to ask on the Discord so other people can see the question and answer as well.

We would also like to reiterate that you should tell us if you spent more than 30 minutes on this homework (like 60+ minutes). The goal of this course is to build understanding, and if you find yourself struggling through the homeworks, then you're probably not understanding something! Rust has a notoriously steep learning curve, so don't be scared to ask for help.