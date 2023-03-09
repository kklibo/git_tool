//! Tests command line invocation of git.

mod common;
mod fixtures;

use crate::common::{match_git_log, Runner};
use std::path::PathBuf;

fn run(repo_dir: PathBuf, parent_hash: &str, section_hash: &str, commit_message: &str) {
    let in_repo_dir = Runner::new(repo_dir);

    in_repo_dir.command("git", &["branch", "parent", parent_hash]);
    in_repo_dir.command("git", &["branch", "section", section_hash]);
    in_repo_dir.command("git", &["branch"]);

    let master_log = in_repo_dir.stdout("git", &["log", "master", "--pretty=format:%s"]);
    let parent_log = in_repo_dir.stdout("git", &["log", "parent", "--pretty=format:%s"]);
    let section_log = in_repo_dir.stdout("git", &["log", "section", "--pretty=format:%s"]);

    assert!(match_git_log(&master_log, &[5, 4, 3, 2, 1]));
    assert!(match_git_log(&parent_log, &[2, 1]));
    assert!(match_git_log(&section_log, &[4, 3, 2, 1]));

    in_repo_dir.command("git", &["checkout", "parent"]);
    in_repo_dir.command("git", &["merge", "--squash", "--no-commit", "section"]);
    in_repo_dir.command("git", &["commit", "-m", commit_message, "--allow-empty"]);
    assert!(in_repo_dir
        .stdout("git", &["diff", "parent", "section"])
        .is_empty());

    in_repo_dir.command("git", &["rebase", "--onto", "parent", "section", "master"]);
}

#[test]
fn test_history_subset_squash() {
    fixtures::test_history_subset_squash(run);
}
