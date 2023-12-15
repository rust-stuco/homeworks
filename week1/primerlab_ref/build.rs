use std::process::Command;

/// Add all patterns here
// const PATTERNS: [&str; 2] = ["src/exercises/fixme*.rs", "src/functions.rs"];
/// Output tar handin file
// const HANDIN: &str = "handin.tar";
const ARGS: [&str; 4] = ["-cvf", "handin.tar", "src/exercises/fixme*.rs", "src/functions.rs"];

fn main() {
    Command::new("tar")
        // .arg("-cvf")
        // .arg(HANDIN)
        // .args(PATTERNS)
        .args(ARGS)
        .output()
        .expect("Unable to tar handin files");

    println!("cargo:rerun-if-changed=handin.tar");
}