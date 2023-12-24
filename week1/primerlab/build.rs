use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 2] = ["src/exercises/fixme*.rs", "src/functions.rs"];

/// Output tar handin file
const HANDIN: &str = "handin.zip";

fn main() {
    Command::new("zip")
        .arg("-r")
        .arg(HANDIN)
        .args(PATTERNS)
        .output()
        .expect("Unable to zip handin files");
}
