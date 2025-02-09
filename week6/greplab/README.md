### 98-008: Intro to the Rust Programming Language

# Grep Lab

The goal of this homework is to make sure you know how to create a command line tool in Rust, as
well as how to write unit tests for your programs.

This homework is going to be _very_ different from other homeworks. There is no autograder, and we
will be manually grading your submission for not just correctness but also code quality and
architecture.

**For this homework, you will follow**
**[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the Rust Book**
**and create a project called `minigrep`**.
Follow the hyperlink and go through all of
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), building `minigrep` as you
go along.

There will be no handout for this homework. Instead, run `cargo new minigrep` in your terminal to
get started!

We encourage you to not skim through the book and blindly copy and paste any code you see. We will
require 2 "extra" things from your submission, which will check if you actually understand what you
are pasting into your code.

# Requirements

**The first requirement** is that you add at least 1 additional feature to `minigrep` on top of the
functionality described in the Book. Given this requirement, it will be more efficient for you to
read the (relatively short) [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
in its entirety, rather than copy and paste everything and then reverse engineer. See the
[Extra Features](#extra-features) section below for some potential features you could add.

**The second requirement** is that you write test cases to test whatever feature you added. We care
more about quality over quantity, but you should have at least 2 types of test cases (general
functionality, edge cases, potentially even performance benchmarking if you want to go above and
beyond with [criterion](https://bheisler.github.io/criterion.rs/book/)).

# Extra Features

The extra feature you want to add is **up to you** once you've finished the base `minigrep`.

-   A very basic feature that you could add is a "count" flag through
    `-c` or `--count`, which changes the output to show how many lines a pattern is in,
    rather than printing out all of the lines.
-   Another feature you could implement is searching directories as well as specific files.
-   You could even add regex support!
    You may want to use the [`regex`](https://docs.rs/regex/latest/regex/) crate for this.

There are many things that you could do here, so try and be creative if you have time!
A good source of inspiration would be the man page for
[`grep`](https://man7.org/linux/man-pages/man1/grep.1.html).

Whatever you choose, add a comment in your `main.rs` file telling us what
feature you added and how to invoke it on the command line.

# Grading

We will be manually grading your submission. We will be looking at he robustness of your code (does
your code run without modifications), correctness of your feature, the quality of your tests,
and general style (documentation and comments).

The way we grade this assignment will be different from almost every Computer Science course here at
CMU, where grading is mainly based on an autograder, and maybe a handful of points deductions for
style. Instead, we will be looking at your code wholistically, judging it based on how people expect
you to write code _outside_ of school.

This practice is taken from CMU's Operating Systems course,
[15-410/615](https://www.cs.cmu.edu/~410/).

# Submission

This time, you will submit the entire `minigrep` crate to Gradescope (instead of just the `src/`
directory)!

Please do not include the `target/` subdirectory when you zip the crate's root folder, either
manually or with `cargo clean`. You can always regenerate it with `cargo build` and `cargo test`.

# Collaboration

In general, feel free to discuss homeworks with other students! As long as you do not copy someone
else's work, any communication is fair game.

All formal questions should be asked on Piazza. Try to discuss on Piazza so that other students can
see your questions and answers as well!

You can also discuss on Discord, but try to keep any technical questions on Piazza.

# Feedback

We would like to reiterate that you should let us know if you spent anywhere in significant excess
of an hour on this homework.

In addition, Rust has a notoriously steep learning curve, so if you find yourself not understanding
the concepts, you should reach out to us and let us know as well --- chances are, you're not the
only one!
