//! Tests command line invocation of git.

use crate::args;
use crate::src::common::{do_commits, match_branch_history, parse_git_log, set_up_repo};
use std::path::PathBuf;
use tempfile::tempdir;

pub fn test_history_subset_squash<T>(to_test: T)
where
    T: Fn(PathBuf, &str, &str, &str),
{
    test_base(to_test, "commit2", "commit4", "commit6");
}

pub fn test_base<T>(f: T, parent_msg: &str, section_msg: &str, commit_message: &str)
where
    T: Fn(PathBuf, &str, &str, &str),
{
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 5);

    let target_branch = "target";
    in_repo_dir.command(&args!["git checkout -b", target_branch]);

    let log_output = in_repo_dir.stdout(&args!["git log --pretty=format:%H\\ %s"]);
    let commits = parse_git_log(&log_output);

    let parent_hash = commits.get(parent_msg).unwrap();
    let section_hash = commits.get(section_msg).unwrap();

    f(
        in_repo_dir.dir.clone(),
        parent_hash,
        section_hash,
        commit_message,
    );

    assert!(match_branch_history(
        &in_repo_dir,
        target_branch,
        &[5, 6, 2, 1]
    ));

    // Confirm archive tag
    let log_output =
        in_repo_dir.stdout(&args!["git log", target_branch, " --pretty=format:%h\\ %s"]);
    let commits = parse_git_log(&log_output);
    let commit_short_hash = commits.get(commit_message).unwrap();
    let tag_name = format!("archive/{}", commit_short_hash.trim());
    assert_eq!(
        in_repo_dir.stdout(&args!["git tag -l", &tag_name]).trim(),
        tag_name
    );
    assert!(match_branch_history(&in_repo_dir, &tag_name, &[4, 3, 2, 1]));

    let branches = in_repo_dir.stdout(&args!["git branch --list parent section"]);
    assert!(branches.is_empty());

    temp_dir.close().unwrap();
}
