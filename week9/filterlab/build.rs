use std::process::Command;

fn main() {
    if cfg!(unix) {
        Command::new("zip")
            .arg("-r")
            .arg("handin.zip")
            .arg("src/")
            .arg("Cargo.toml")
            .output()
            .expect("Unable to zip handin files");
    }

    println!("cargo:rerun-if-changed=handin.zip");
}
