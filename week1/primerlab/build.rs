use std::fs;
use std::io::prelude::*;
use std::process::Command;

// Add all patterns here
const PATTERNS: [&str; 2] = ["src/exercises/fixme*.rs", "src/functions.rs"];

fn main() {
    // Create the shell script no matter what OS this is running on
    create_submit_script().expect("Unable to create submit.sh script");

    // Automatically create handin for unix
    if cfg!(unix) {
        tar_submission();
    } else if cfg!(windows) {
        todo!("Auto generate handin.tar for windows is not yet implemented")
    }

    // Will rerun this file every time code in this crate is changed
    println!("cargo:rerun-if-changed=build.rs");
}

// All of these functions _shouldn't_ fail, unless the user doesn't have correct permissions
fn create_submit_script() -> Result<(), std::io::Error> {
    let mut file = fs::File::create("submit.sh")?;

    let mut script = String::from("tar -cvf handin.tar");
    for pattern in PATTERNS {
        script.push(' ');
        script.push_str(pattern);
    }

    file.write_all(script.as_bytes())?;

    Ok(())
}

fn tar_submission() {
    // check if tar is actually installed
    let tar_status = Command::new("which")
        .arg("tar")
        .status()
        .expect("`which` failed to execute");

    if let Some(0) = tar_status.code() {
        Command::new("sh")
            .arg("submit.sh")
            .output()
            .expect("Failed to tar submission files");

        // Only remove submit.sh if we successfully created handin.tar, since we don't need it
        fs::remove_file("submit.sh").expect("Unable to remove submit.sh");
    }
}