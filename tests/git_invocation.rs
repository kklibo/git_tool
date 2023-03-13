//! Tests equivalent functionality through command line git invocation.

mod src;

use src::common::match_branch_history;
use src::fixtures;
use src::runner::Runner;
use std::path::PathBuf;

fn run(repo_dir: PathBuf, parent_hash: &str, section_hash: &str, commit_message: &str) {
    let in_repo_dir = Runner::new(repo_dir);

    in_repo_dir.command(&["git", "branch", "parent", parent_hash]);
    in_repo_dir.command(&["git", "branch", "section", section_hash]);

    // Test: confirm expected branch history
    assert!(match_branch_history(
        &in_repo_dir,
        "master",
        &[5, 4, 3, 2, 1]
    ));
    assert!(match_branch_history(&in_repo_dir, "parent", &[2, 1]));
    assert!(match_branch_history(&in_repo_dir, "section", &[4, 3, 2, 1]));

    in_repo_dir.command(&["git", "checkout", "parent"]);
    in_repo_dir.command(&["git", "merge", "--squash", "--no-commit", "section"]);
    in_repo_dir.command(&["git", "commit", "-m", commit_message, "--allow-empty"]);
    assert!(in_repo_dir
        .stdout("git", &["diff", "parent", "section"])
        .is_empty());

    in_repo_dir.command(&["git", "rebase", "--onto", "parent", "section", "master"]);
}

#[test]
fn test_history_subset_squash() {
    fixtures::test_history_subset_squash(run);
}
