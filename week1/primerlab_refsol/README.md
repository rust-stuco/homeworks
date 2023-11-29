### 98-008: Intro to the Rust Programming Language



# Primer Lab

The goal of this homework is to make sure that you understand Rust's basic syntax.

We recommend you make use of the [Rust Book](https:doc.rust-lang.org/book/title-page.html)
chapters 1-3, as the "textbook" of this course. It's generally easy to follow along with,
and would make handy reference material for the homeworks in this course.

We've tried to calibrate this homework to take around an hour,
so if you are spending much more than that, please let us know!
This time limit does not include setting up Rust on your machine though,
so if you have trouble with installing Rust, make sure to ask for help.



# Setup

Make sure you have Rust installed. You should have installed Rust through `rustup`.
To confirm, run:

```sh
$ rustup --version
rustup 1.26.0 (2023-11-14)
$ cargo --version
cargo 1.74.0 (ecb9851af 2023-10-18)
```

Make sure that both of those commands execute successfully,
and that they have relatively recent dates.
If the dates are not recent, you can update `rustup` by running `rustup update`.

Once you have `cargo` installed, run `cargo doc --open` in this directory to generate documentation
for this homework.

```sh
$ cargo doc --open
Documenting primerlab v0.1.0 (<path>/primerlab)
   Finished dev [unoptimized + debuginfo] target(s) in 0.11s
    Opening <path>/primerlab/target/doc/primerlab/index.html
```

`cargo doc` will generate an HTML file that you can view in your browser!
It might be easier to read the markdown-rendered docs than the comments in this handout.



# Part 1: Exercises

For the first part of this homework, there are 8 files that do not compile under `src/exercises`.
You will need to modify each of them in some way to make them compile.

Run this command in the root of your project (parent directory of the `src/` folder):
```sh
$ cargo test
```
**What does the error say?** Go into `src/exercises/fixme1.rs` and make the change.

When you are able to run `cargo test` without any errors,
move on to the next exercise by uncommenting `pub mod fixme2` in `src/exercises/mod.rs`.
Go through all 8 of the exercises by uncommenting each of the `pub mod fixme_;`s,
and make sure that all the `fixme_` test cases pass when you run `cargo test`.

At this point, you'll see some other tests failing. You'll fix those in the next section!



# Part 2: Function implementations

You will now need to implement 4 relatively simple functions in Rust.
In `src/functions.rs`, you will find 4 functions with a `todo!()` inside of them.
Replace that `todo!()` with your implementation, according to the comment specification.

_One of the functions requires you to implement it in a certain way,_
_so make sure to read those comments carefully._

To test all of your functions, run:
```sh
$ cargo test
```

Note that `cargo test` will run the tests under `src/tests.rs`, as well as run each of the
code examples in the comments.

If you find that a test is running slowly, run `cargo test --release`,
which runs the tests in release mode. This runs the tests with
compiler optimizations (like the `-O2` flag for C) and without debug symbols.
Also, if you want to run a specific test like `fn it_works()`, run `cargo test -- it_works`
with the name of the test instead of `it_works`.



# Submission
- Run `make handin`
- Submit the `handin.tar` that was generated to Gradescope.

If you do not have `make` and `tar` installed on your system,
install `make` and `tar` on your machine or use the CMU Linux SSH machines.
If you need help with this, please reach out to us!



# Collaboration

In general, feel free to discuss homeworks with other students!
As long as you do not copy someone else's work, any communication is fair game.
Try to discuss on the course Discord or Piazza so that
other students can see your questions and answers as well!



# Feedback

We would like to reiterate that you should let us know if you spent
anywhere in significant excess of an hour on this homework.
These assignments are being deployed for the first time,
and we are definitely open to feedback regarding the length and difficulty.
In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!
