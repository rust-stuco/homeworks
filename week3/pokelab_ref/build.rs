use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 2] = ["src/pokemon/charmander.rs", "src/pokemon/eevee.rs"];

fn main() {
    Command::new("zip")
        .arg("handin.zip")
        .args(PATTERNS)
        .output()
        .expect("Unable to zip handin files");
}
