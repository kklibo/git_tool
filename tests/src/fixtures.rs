//! Tests command line invocation of git.

use crate::args;
use crate::src::common::{do_commits, match_branch_history, parse_git_log, set_up_repo};
use std::path::PathBuf;
use tempfile::tempdir;

pub fn test_history_subset_squash<T>(f: T)
where
    T: Fn(PathBuf, &str, &str, &str),
{
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 5);

    let log_output = in_repo_dir.stdout(&args!["git log --pretty=format:%H\\ %s"]);
    let commits = parse_git_log(&log_output);

    let parent_hash = commits.get("commit2").unwrap();
    let section_hash = commits.get("commit4").unwrap();

    f(
        in_repo_dir.dir.clone(),
        parent_hash,
        section_hash,
        "commit6",
    );

    assert!(match_branch_history(&in_repo_dir, "master", &[5, 6, 2, 1]));
    assert!(match_branch_history(&in_repo_dir, "parent", &[6, 2, 1]));
    assert!(match_branch_history(&in_repo_dir, "section", &[4, 3, 2, 1]));

    temp_dir.close().unwrap();
}
