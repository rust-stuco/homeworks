### 98-008: Intro to the Rust Programming Language

_A reminder that if you did the challenge from last homework, you don't need to do this one._
_Follow the instructions in the [Submission](#submission) section,_
_but submit your CLI tool code instead of this and give us a heads up that you did that!_



# Grep Lab

The goal of this homework is to make sure you know how to create a command line tool in Rust,
as well as how to write unit tests for your programs.

This homework is going to be _very_ different from other homeworks.
There is no autograder, and we will probably only
[grade your code](#grading) at the end of the semester.


**For this homework, you will follow**
**[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the Rust Book**
**and create a project called `minigrep`**.
Follow the hyperlink and go through all of
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html),
building `minigrep` as you go along.

There will be no handout for this homework. Intead, run `cargo new minigrep` to get started!

We encourage you to not skim through the book and blindly copy and paste any code you see.
Of course, we cannot enforce this, so it's basically up to you
how much you want to get out of this homework.

We will require 2 things, both explained the [Requirements](#requirements) section.
But first, a bit about [grading](#grading).



# Grading

**This homework will not be graded until the end of the semester.**
More specifically, if a student is under the 60% threshold needed to pass this course,
we will manually go through their submission for this homework
and see if we can increase their grade.

Since we expect most students to get above 60% (since 50% comes from required attendance,
and you automatically fail if you don't show up 3 times),
we will not manually grade everyone's submission.

If you have already completed all of the previous homeworks,
and you know you will show up to enough lectures, then you don't actually need to do this homework.
That being said, we think this is still a valuable exercise, especially since
we provided all of the boilerplate, documentation, and testing for you in the previous homeworks.

If you have the time, and you want to get better at Rust,
we encourage you to still go through with this! If you don't, then no worries.
You will still have the opportunity to get experience creating your own crate in the final project.

---

If we do grade your submission (at the end of the semester),
we will grade on the robustness of your code (does your code run without modifications),
correctness of your feature, the quality of your tests,
and general style (documentation and comments).

_If you would like us to review your code as you submit it (in preparation for your final project),_
_just let us know and we can make an exception!_

The next section will explain what the [requirements](#requirements) are for this homework.



# Requirements

**The first requirement** is that you add at least 1 additional feature to `minigrep` on top of the
functionality described in the Book. Given this requirement,
it might actually be more efficient for you to read the (relatively short)
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
in its entirety, rather than copy and paste everything and then reverse engineer.
See the [Extra Features](#extra-features) section below for some potential features you could add.

**The second requirement** is that you write test cases to test whatever feature you added.
Try to write at least 2 test cases
(1 for general functionality, 1 for either edge cases or performance).



# Extra Features

The extra feature you want to add is **up to you** once you've finished the base `minigrep`.

- A very basic feature that you could add is a "count" flag through
`-c` or `--count`, which changes the output to show how many lines a pattern is in,
rather than printing out all of the lines.

- Another feature you could implement is searching directories as well as specific files.

- You could even add regex support!
You may want to use the [`regex`](https://docs.rs/regex/latest/regex/) crate for this.

There are many things that you could do here, so try and be creative if you have time!
A good source of inspiration would be the man page for
[`grep`](https://man7.org/linux/man-pages/man1/grep.1.html).

Whatever you choose, add a comment in your `main.rs` file telling us what
feature you added and how to invoke it on the command line.



# Submission

This time, you will submit the entire `minigrep` crate to Gradescope
(instead of just the `src/` directory)!

Please delete the `target/` subdirectory before you zip the crate's root folder,
either manually or with `cargo clean`. Keeping that there will just add size.
You can always regenerate it with `cargo build` and `cargo test`.

_If you did the challenge from last week, please submit your crate_
_in the same way, removing all temporary and uneccessary files._



# Collaboration

In general, feel free to discuss homeworks with other students!
As long as you do not copy someone else's work, any communication is fair game.

All formal questions should be asked on Piazza. Try to discuss on Piazza so that
other students can see your questions and answers as well!

You can also discuss on Discord, but try to keep any technical questions on Piazza.



# Feedback

We would like to reiterate that you should let us know if you spent
anywhere in significant excess of an hour on this homework.

In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!
