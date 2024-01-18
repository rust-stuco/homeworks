### 98-008: Intro to the Rust Programming Language



# Get Owned Lab

The goal of this homework is to familiarize yourself with Rust's system of Ownership.

Lecture was heavily based on
[Chapter 4 of the Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
We **_strongly_** suggest that you make sure you understand the rules of Ownership
before attempting this homework, and this text is a great place to reference!

This homework is a bit shorter than the previous one, mainly so you can have extra time to
read through the book and stew with the concepts a bit.

If you find yourself getting stuck,
it might be good to take a small break and go over the borrow checker's
[rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules).

Remember to always read the error messages that the compiler gives you, and as always,
we encourage you to reach out for help!



# Part 1: Slices

Like the previous homework, you will modify 3 files that don't compile by
adding, removing, or reorganizing code to make them compile!
The process should be identical to the previous homework,
except the exercises are now specific to slices.

When you are done, you should be able to run `cargo test` with all of the
modules (`pub mod`s) in `src/slices/mod.rs` uncommented.



# Part 2: Move Semantics

Like the previous section, you will modify 5 files to make them compile.
The first 3 exercises involve the `Vec` type, which is an _owned_ type.

For this homework, all you need to know is the `vec![]` macro and the `push` method on vectors.
Recall the `vec![]` creates a new vector with elements, and
`push` appends an element onto the end of a vector.



# Part 3: Strings

In this section, you'll be working with `String` and `&str` instead of `Vec`.
Recall that `String` is an _owned_ type and `&str` is _borrowed_ type.

The first three exercises in `src/strings` are just like the previous section,
where you need to make some changes for the functions to compile.

_A useful thing to remember here is that a `&String` can be deref coerced into a `&str`._
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


### Unix

If you are on a unix system, we will try to create a `handin.zip` automatically for you,
**but you will need to have `zip` already installed**.

If you _do not_ have `zip` installed on your system,
install `zip` on your machine or use the CMU Linux SSH machines.
If you need help with this, please reach out to us!

Once you have `zip` installed, we will create the `handin.zip` automatically for you
(_take a peek into `build.rs` if you're interested in how this works!_).

Once you have the `handin.zip` file, submit it to Gradescope.


### Windows

If you are on a windows system, you can zip the `src/` folder manually
and upload that to Gradescope.

Note that you don't _need_ to name it `handin.zip`, you can name it whatever you'd like.



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
