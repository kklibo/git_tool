//! Tests command line invocation of git.

mod src;

use assert_cmd::prelude::*;
use src::common::{do_commits, match_git_log, parse_git_log, set_up_repo};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_history_subset_squash() {
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 5);

    let log_output = in_repo_dir.stdout("git", &["log", "--pretty=format:%H %s"]);
    let commits = parse_git_log(&log_output);

    let parent_hash = commits.get("commit2").unwrap();
    let section_hash = commits.get("commit4").unwrap();

    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(&in_repo_dir.dir)
        .args(&[parent_hash, section_hash, "commit6"])
        .output()
        .unwrap();

    println!("{:#?}", output);

    let master_log = in_repo_dir.stdout("git", &["log", "master", "--pretty=format:%s"]);
    let parent_log = in_repo_dir.stdout("git", &["log", "parent", "--pretty=format:%s"]);
    let section_log = in_repo_dir.stdout("git", &["log", "section", "--pretty=format:%s"]);

    assert!(match_git_log(&master_log, &[5, 6, 2, 1]));
    assert!(match_git_log(&parent_log, &[6, 2, 1]));
    assert!(match_git_log(&section_log, &[4, 3, 2, 1]));

    temp_dir.close().unwrap();
}
