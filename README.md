# Rust StuCo S24 Homeworks

_All homeworks are due the week after they are assigned, unless you use late days._
_You have 7 late days to use whenever you need._



# Homeworks

We've' tried to calibrate each of the homeworks to take around an hour.
If you find yourself spending much more than an hour on these, please let us know!
This is the first time these assignments have been handed out,
and we are very open to feedback regarding the length and difficulty.



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



# Submission

We will try to create a `handin.tar` automatically for you,
**but you will need to have `tar` already installed**.

If you _do not_ have `tar` installed on your system,
install `tar` on your machine or use the CMU Linux SSH machines.
If you need help with this, please reach out to us!

Once you have `tar` installed, we will create the `handin.tar` automatically for you
(_take a peek into `build.rs` if you're interested in how this works!_).

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

In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!