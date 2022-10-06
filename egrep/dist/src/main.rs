// Turn on some helpful warnings
#![warn(rust_2018_idioms)]
// Turns off the dead code warning for now. Once you start calling functions, re-enabling might be
// a good idea, but it's way too noisy to start!
#![allow(dead_code)]

use std::{
    io::{self, BufRead},
    process::ExitCode,
};

mod matcher;

/// Prints a short string indicating the correct usage of the binary.
fn usage(name: &'_ str) {
    eprintln!("Usage: {} PATTERN [FILE]...", name);
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();

    match args.as_slice() {
        [] => usage("egrep"),
        [name] => usage(name),
        // You can take a quick peek at
        // <https://doc.rust-lang.org/reference/patterns.html#slice-patterns> or ask in Discord if
        // you want this syntax explained but we're essentially ignoring the first argument,
        // calling the second one `pattern : String`, and calling the rest (possibly empty)
        // `files : &[String]`.
        [_, pattern, files @ ..] => todo!(),
    }

    // Exit with success. For failure you can use `std::process::ExitCode::FAILURE`.
    std::process::ExitCode::SUCCESS
}

fn get_files<'a>(
    files: impl Iterator<Item = &'a str>,
) -> Vec<io::Result<(&'a str, Box<dyn BufRead>)>> {
    todo!("A function like this might be useful to deal with both files and stdin")
}
