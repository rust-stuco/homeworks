// Turn on some helpful warnings
#![warn(rust_2018_idioms)]

use std::{
    io::{self, BufRead},
    process::ExitCode,
};

use matcher::Matcher;

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
        [_, pattern, files @ ..] => {
            let file_handles = if files.is_empty() {
                get_files(std::iter::once("-"))
            } else {
                get_files(files.iter().map(String::as_str))
            };

            let print_filename = file_handles.len() > 1;
            let matcher = match Matcher::new(pattern) {
                Ok(m) => m,
                Err(_) => {
                    eprintln!("Invalid pattern syntax");
                    return std::process::ExitCode::FAILURE;
                }
            };

            for handle in file_handles {
                match handle {
                    Ok((name, handle)) => {
                        for line in handle.lines() {
                            let line = match line {
                                Ok(line) => line,
                                Err(_) => {
                                    eprintln!("Error reading from file {}", name);
                                    break;
                                }
                            };

                            if matcher.matches(&line) {
                                if print_filename {
                                    println!("{}:{}", name, line)
                                } else {
                                    println!("{}", line)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }

    std::process::ExitCode::SUCCESS
}

fn get_files<'filename>(
    files: impl Iterator<Item = &'filename str>,
) -> Vec<io::Result<(&'filename str, Box<dyn BufRead>)>> {
    files
        .map(|file| {
            if file == "-" {
                Ok(("(standard input)", Box::new(std::io::stdin().lock()) as _))
            } else {
                std::fs::File::open(file).map(|f| (file, Box::new(std::io::BufReader::new(f)) as _))
            }
        })
        .collect()
}
