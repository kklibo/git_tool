//! Tests command line invocation of git.

mod src;

use assert_cmd::prelude::*;
use src::fixtures;
use std::path::PathBuf;
use std::process::Command;

fn run(repo_dir: PathBuf, parent_hash: &str, section_hash: &str, commit_message: &str) {
    let _output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(repo_dir)
        .args(&[parent_hash, section_hash, commit_message])
        .output()
        .unwrap();
}

#[test]
fn test_history_subset_squash() {
    fixtures::test_history_subset_squash(run);
}
