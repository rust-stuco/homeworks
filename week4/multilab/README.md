### 98-008: Intro to the Rust Programming Language



# Multi Lab

The goal of this homework is to make sure you are comfortable with some of the
generic collection types in the [`std::collections`] module.

You will implement two data structures that track multiplicity (the number of times
elements appear) on top of their normal operations.
The first will be a [`MultiSet`](crate::multiset::MultiSet),
which is a set that can have duplicates.
The second will be a [`MultiMap`](crate::multimap::MultiMap), which is a map that maps keys
to any number of values.

For [`MultiSet`](crate::multiset::MultiSet), we have not provided what types the fields should be.
_We recommend that you use one of the collection types provided in the_
_[`std::collections`] module to help you build these new types._

We would like you think a bit more about your own code than previous homeworks,
so we will be providing less detail than before on implementation details.
Instead, you should read the documentation and implement the data structure
and methods so that they follow the documentation and pass all the test cases.



# Part 1: `MultiSet`

Click this hyperlink to go to the documentation for [`MultiSet`](crate::multiset::MultiSet)!



# Part 2: `MultiMap`

Click this hyperlink to go to the documentation for [`MultiMap`](crate::multimap::MultiMap)!



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
