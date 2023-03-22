mod src;

use assert_cmd::prelude::*;
use src::fixtures;
use std::path::PathBuf;
use std::process::Command;

fn run(repo_dir: PathBuf, parent_hash: &str, section_hash: &str, commit_message: &str) {
    let _output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(repo_dir)
        .args(&[parent_hash, section_hash, commit_message, "--verbose"])
        .output()
        .unwrap();

    dbg!(&_output);
    println!("{}", String::from_utf8(_output.stdout).unwrap());
}

#[test]
fn test_history_subset_squash() {
    fixtures::test_history_subset_squash(run);
}
