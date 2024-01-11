### 98-008: Intro to the Rust Programming Language



# Summary Lab

The goal of this homework is to make sure you understand error handling and the `?` operator,
as well as traits and how to implement them. While these are the main topics for this week,
this homework will also heavily test your fluency with Rust's [`String`] and its methods.

You will be implementing two traits on two different types. The two traits are
[`Reader`](crate::Reader) and [`Summary`](crate::Summary), and the two types
are [`EmailReader`](crate::reader::email_reader::EmailReader) and
[`TweetReader`](crate::reader::tweet_reader::TweetReader).

Since string parsing is a bit tricky (especially in Rust), you will get `70` points for completing
the implementation for [`EmailReader`](crate::reader::email_reader::EmailReader)
and `80` points for completing the implementation for
[`TweetReader`](crate::reader::tweet_reader::TweetReader).

If you find yourself spending more than an hour on this assignment, please let us know!
This assignment is harder than the previous homeworks,
so there is no shame if you can't complete it in under an hour.

Also, remember that there are plenty of points to be earned, and if you show up to every lecture,
that is already `50%` of your grade (and you need `60%` to pass)!

That being said, if you have the time,
we still encourage you to try and pass all of the test cases for practice!

### Command Line Tool Challenge

Another thing you can do is turn this crate into a binary that _actually_
reads from the file system.

Right now, the `new` functions on each of the readers never actually get called because
there is no `main` function to call them. If you're up to the challenge,
create a command line tool that allows you to read in a tweet or an email from the `data` folder
and summarize what is reads.

You can decide what flags to pass in for specific outputs. And if you'd like, you can
take a look at the [clap](https://docs.rs/clap/latest/clap/index.html) third-party crate
to help you!

To reiterate: Only do this if you have ample time to spare and you want to get better at Rust!
This is not a trivial task.
We also don't have an autograder set up for this, so you'll mostly be on your own.
If you do decide to do this, we'll manually grade it ourselves to determine how many points
you will receive.



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

If you want a local version of this writeup, you can generate it with `cargo doc`.
Once you have `cargo` installed, run `cargo doc --open` in this directory to generate documentation
for this homework.

```sh
$ cargo doc --open
Documenting primerlab v0.1.0 (<path>/primerlab)
   Finished dev [unoptimized + debuginfo] target(s) in 0.11s
    Opening <path>/primerlab/target/doc/primerlab/index.html
```

Either way, a version of this writeup will be up on our website!




# Part 1: Emails

Click this hyperlink to go to the writeup for
[`email_reader`](crate::reader::email_reader)!



# Part 2: Tweets

Click this hyperlink to go to the writeup for
[`tweet_reader`](crate::reader::tweet_reader)!



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

Try to discuss on the course Discord or Piazza so that
other students can see your questions and answers as well!

Remember that all formal questions should be asked on Piazza.



# Feedback

We would like to reiterate that you should let us know if you spent
anywhere in significant excess of an hour on this homework.

In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!
