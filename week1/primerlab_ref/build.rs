use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 2] = ["src/exercises", "src/functions.rs"];

fn main() {
    // For some reason zip doesn't like wildcards when run through this
    // So we have to include the entire src/exercises directory and then exclude the mod.rs file
    Command::new("zip")
        .arg("-r")
        .arg("handin.zip")
        .args(PATTERNS)
        .arg("-x")
        .arg("src/exercises/mod.rs")
        .output()
        .expect("Unable to zip handin files");
}
