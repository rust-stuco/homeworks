### 98-008: Intro to the Rust Programming Language



# Get Owned Lab

The goal of this homework is to familiarize yourself with Rust's system of Ownership.

Lecture was heavily based on Chapter 4 of the
[Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
We **_strongly_** suggest that you make sure you understand the rules of Ownership
before attempting this homework, and this text is a great place to reference!

This homework is a bit shorter than the previous one, mainly so you can have extra time to
read through the book and stew with the concepts a bit.

If you find yourself getting stuck,
it might be good to take a small break and just go over the borrow checker's rules.

Remember to always read the error message that the compiler gives you, and as always,
we encourage you to reach out for help!



# Part 1: Slices

Like the previous homework, you will modify 3 files that don't compile by changing a few things!
This is basically identical to the previous homework, except the exercises are specific to slices.

When you are done, you should be able to run `cargo test` with all the
modules (`pub mod`s) in `src/slices/mod.rs` uncommented.



# Part 2: Move Semantics

Like the previous section, you will modify 5 files that don't compile.
The first 3 exercises involve the `Vec` type, which is an _owned_ type.

For this homework, all you need to know is the `vec![]` macro and the `push` method on vectors.
Recall the `vec![]` creates a new vector with elements, and
`push` appends an element onto the end of a vector.



# Part 3: Strings

In this section, you'll be working with `String` and `&str` instead of `Vec`.
Recall that `String` is an _owned_ type and `&str` is _borrowed_ type.

The first three exercises in `src/strings` are just like the previous section,
where you need to make some changes for the functions to compile.

_A useful thing to remember here is that a `&String` can be coerced into a `&str`._
_Other than that, just remember to always read what the compiler tells you_
_(it may or may not give away the answer)!_

For `strings4.rs`, you will need to look at some real documentation.
You _could_ just google the answer,
but it would be good to familiarize yourself with official Rust documentation!
Linked here is the documentation for
[`str`](https://doc.rust-lang.org/std/primitive.str.html), which might be useful.

There's a search bar at the top of the documentation that you can also make use of.
Try searching for methods called `trim` there!



# Submission
We will try to create a `handin.tar` automatically for you,
**but you will need to have `tar` already installed**.

If you _do not_ have `tar` installed on your system,
install `tar` on your machine or use the CMU Linux SSH machines.
If you need help with this, please reach out to us!

You can also submit manually.
If `tar` is not installed, a `submit.sh` file should be created with the following inside it
(you can run this manually if you want):

```
tar -cvf handin.tar src/exercises/fixme*.rs src/functions.rs
```

If you _do_ have `tar` already installed, we will create the `handin.tar` automatically for you
(_take a peek into `build.rs` if you're interested in how!_).

Once you have the `handin.tar`, submit it to Gradescope.



# Collaboration

In general, feel free to discuss homeworks with other students!
As long as you do not copy someone else's work, any communication is fair game.

Try to discuss on the course Discord or Piazza so that
other students can see your questions and answers as well!

Remember that all formal questions should be asked on Piazza.



# Feedback

We would like to reiterate that you should let us know if you spent
anywhere in significant excess of an hour on this homework.

These assignments are being deployed for the first time,
and we are very open to feedback regarding the length and difficulty.

In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!