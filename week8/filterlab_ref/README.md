# Rust StuCo S24 Homeworks

_All homeworks are due the week after they are assigned, unless you use late days._
_You have 7 late days to use whenever you need._


# Filter Lab

The goal of this homework is to tie together several of the things we have talked about over the
past few weeks. This includes code-related topics such as structs, standard collections, and
generics, but we also want to make sure you understand some of the development tools that Rust
provides, such as crates, modules, and libraries.

For this lab, we will not be giving a handout. Instead, we would like you to create your own crate
(without any of our starter code). If you have trouble setting things up, you can always reference
the reference solution on our [GitHub](todo_link).


# Bloom Filters

For this homework, you will get to implement a bloom filter! Bloom filters are arguably one of the
most important probabilistic data structures in all of computer systems (and perhaps even computer
science).

TODO(cjtsui):
- How much explanation do we need to give for bloom filters, and where should we link for more
details?
- At a bare minimum explain the high level concepts
- Since we require 15-122, we don't need to explain a bitvector, but we can still point them to
online resources if they forgot

One final thing to note before you get started: we are going to ask you to complete another
programming assignment that will make use of the bloom filter you program for this assignment.
In that future programming assignment, we will ask you to make the bloom filter thread-safe.
In other words, a multi-threaded and safe-by-default implementation of a bloom filter!


# Grading

TODO(cjtsui):
- Do we give a handout or do we ask them to copy and paste the extra files from github?
- A handout means they need to copy and paste the files anyways


# Leaderboard

TODO(cjtsui):
- Actually implement the leaderboard on Gradescope




# Submission

### Formatting and Style

The autograder will run these two commands on your code:

```sh
cargo clippy && cargo fmt --all -- --check
```

If the autograder detects any errors from the command above,
you will not be able to receive any points. This may seem strict, but we have decided to follow
standard best practices for Rust.

By following [Rust's style guidelines](https://doc.rust-lang.org/stable/style-guide/),
you ensure that anybody reading your code (who is familiar with Rust) will be
able to easily navigate your code. This can help with diving into an unfamiliar code base,
and it also eliminates the need for debate with others over style rules, saving time and energy.

See the official [guidelines](https://doc.rust-lang.org/stable/style-guide/) for more information.

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
