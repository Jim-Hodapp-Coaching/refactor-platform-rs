use std::io::{self, Write};
use std::process::Command;

fn main() {
    let workspace_root = ".".to_string();

    for crate_name in find_crates(&workspace_root).iter() {
        let mut command = Command::new("cargo");
        println!("Running Tests for {:?} Crate", crate_name);
        command
            .args(["test", "--features", "mock"])
            .args(["-p", crate_name]);
        let output = command.output().unwrap();

        println!("Test Output Status: {}", output.status);

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

fn find_crates(workspace_root: &str) -> Vec<String> {
    let mut crates = Vec::new();

    for entry in std::fs::read_dir(workspace_root).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() && entry.path().join("Cargo.toml").is_file() {
            let crate_name = entry.file_name().to_string_lossy().to_string();
            crates.push(crate_name);
        }
    }

    crates
}
