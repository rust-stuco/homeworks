use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 3] = [
    "src/slices/slices*.rs",
    "src/move_semantics/move_semantics*.rs",
    "src/strings/strings*.rs",
];

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
