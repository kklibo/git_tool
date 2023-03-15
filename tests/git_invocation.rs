//! Tests equivalent functionality through command line git invocation.

mod src;

use src::common::match_branch_history;
use src::fixtures;
use src::runner::Runner;
use std::path::PathBuf;

fn run(repo_dir: PathBuf, parent_hash: &str, section_hash: &str, commit_message: &str) {
    let in_repo_dir = Runner::new(repo_dir);

    let target_branch = in_repo_dir.stdout(&args!["git branch --show-current"]);
    let target_branch = target_branch.trim();

    in_repo_dir.command(&args!["git branch parent", parent_hash]);
    in_repo_dir.command(&args!["git branch section", section_hash]);

    // Test: confirm expected branch history
    assert!(match_branch_history(
        &in_repo_dir,
        target_branch,
        &[5, 4, 3, 2, 1]
    ));
    assert!(match_branch_history(&in_repo_dir, "parent", &[2, 1]));
    assert!(match_branch_history(&in_repo_dir, "section", &[4, 3, 2, 1]));

    in_repo_dir.command(&args!["git checkout parent"]);
    in_repo_dir.command(&args!["git merge --squash --no-commit section"]);
    in_repo_dir.command(&args!["git commit -m", commit_message, "--allow-empty"]);

    let diff = in_repo_dir.stdout(&args!["git diff parent section"]);
    assert!(diff.is_empty());

    in_repo_dir.command(&args!["git rebase --onto parent section", target_branch]);

    let parent_short_hash = in_repo_dir.stdout(&args!["git show parent --format=%h"]);
    let tag_name = format!("archive/{}", parent_short_hash.trim());
    in_repo_dir.command(&args!["git tag ", &tag_name, "section"]);

    // This should be redundant: the `rebase --onto` operation should do this.
    in_repo_dir.command(&args!["git checkout", target_branch]);
}

#[test]
fn test_history_subset_squash() {
    fixtures::test_history_subset_squash(run);
}
