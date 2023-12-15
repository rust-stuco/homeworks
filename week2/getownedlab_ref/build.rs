use std::fs;
use std::io::prelude::*;
use std::process::Command;

/// Add all patterns here
const PATTERNS: [&str; 3] = [
    "src/slices/slices*.rs",
    "src/move_semantics/move_semantics*.rs",
    "src/strings/strings*.rs"
];
/// Temp file to run tar command
const SHELL_SCRIPT: &str = "handin.sh";
/// Output tar handin file
const HANDIN: &str = "handin.tar";

fn main() {
    // Create the shell script no matter what
    create_handin_script().expect(&format!("Unable to create {} script", SHELL_SCRIPT));

    // Automatically create handin, delete shell script if successful
    tar_handin();
}

// All of these functions _shouldn't_ fail, unless the user doesn't have correct permissions
fn create_handin_script() -> Result<(), std::io::Error> {
    let mut file = fs::File::create(SHELL_SCRIPT)?;

    let mut script = format!("tar -cvf {}", HANDIN);
    for pattern in PATTERNS {
        script.push(' ');
        script.push_str(pattern);
    }

    file.write_all(script.as_bytes())?;

    Ok(())
}

fn tar_handin() {
    // check if tar is actually installed
    let tar_status = Command::new("which")
        .arg("tar")
        .status()
        .expect("`which` failed to execute");

    // If tar is found then create the handin
    if let Some(0) = tar_status.code() {
        Command::new("sh")
            .arg(SHELL_SCRIPT)
            .output()
            .expect("Failed to tar handin files");

        // Only remove handin.sh if we successfully created handin.tar, since we don't need it
        fs::remove_file(SHELL_SCRIPT).expect(&format!("Unable to remove {}", SHELL_SCRIPT));
    }
}

