use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 2] = ["src/exercises/fixme*.rs", "src/functions.rs"];

/// Output tar handin file
const HANDIN: &str = "handin.tar";

fn main() {
    Command::new("tar")
        .arg("-cvf")
        .arg(HANDIN)
        .args(PATTERNS)
        .output()
        .expect("Unable to tar handin files");
}