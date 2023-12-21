use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut exit_codes = Vec::new();

    for crate_name in crates_to_test().iter() {
        let mut command = Command::new("cargo");

        println!("Running Tests for {:?} Crate", crate_name);

        // It may be that we need to map each crate with specific commands at some point
        // for now calling "--features mock" for each crate.
        command
            .args(["test", "--features", "mock"])
            .args(["-p", crate_name]);

        let output = command.output().unwrap();

        println!("Test Output Status: {}", output.status);

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        exit_codes.push(output.status.code().unwrap());
    }
    if exit_codes.iter().any(|code| *code != 0i32) {
        println!("Exit Codes From Tests: {:?}", exit_codes);
        // Will fail CI
        std::process::exit(1);
    }
    // Will pass CI
    std::process::exit(0);
}

fn crates_to_test() -> Vec<String> {
    vec!["entity_api".to_string(), "web".to_string()]
}
