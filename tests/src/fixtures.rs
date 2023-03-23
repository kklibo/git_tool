//! Tests command line invocation of git.

use crate::args;
use crate::src::common::{do_commits, match_branch_history, parse_git_log, set_up_repo};
use crate::src::runner::Runner;
use std::path::PathBuf;
use tempfile::tempdir;

pub fn test_history_subset_squash<T>(to_test: T)
where
    T: Fn(PathBuf, &str, &str, &str),
{
    test_base(
        to_test,
        |runner| do_commits(runner, 5),
        "commit2",
        "commit4",
        "commit6",
        &[5, 6, 2, 1],
        &[4, 3, 2, 1],
    );
}

pub fn test_history_subset_squash_from_head<T>(to_test: T)
where
    T: Fn(PathBuf, &str, &str, &str),
{
    test_base(
        to_test,
        |runner| do_commits(runner, 5),
        "commit2",
        "commit5",
        "commit6",
        &[6, 2, 1],
        &[5, 4, 3, 2, 1],
    );
}

pub fn test_base<T, U>(
    f: T,
    populate_repo: U,
    parent_msg: &str,
    section_msg: &str,
    commit_message: &str,
    expected_branch_history: &[usize],
    expected_tag_history: &[usize],
) where
    T: Fn(PathBuf, &str, &str, &str),
    U: Fn(&Runner),
{
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    let target_branch = "target";
    in_repo_dir.command(&args!["git checkout -b", target_branch]);

    populate_repo(&in_repo_dir);

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
        expected_branch_history
    ));

    // Confirm archive tag + history
    let log_output =
        in_repo_dir.stdout(&args!["git log", target_branch, " --pretty=format:%h\\ %s"]);
    let commits = parse_git_log(&log_output);
    let commit_short_hash = commits.get(commit_message).unwrap();
    let tag_name = format!("archive/{}", commit_short_hash.trim());
    assert_eq!(
        in_repo_dir.stdout(&args!["git tag -l", &tag_name]).trim(),
        tag_name
    );
    assert!(match_branch_history(
        &in_repo_dir,
        &tag_name,
        expected_tag_history
    ));

    // Confirm temp working branches have been deleted
    let branches = in_repo_dir.stdout(&args!["git branch --list parent section"]);
    assert!(branches.is_empty());

    temp_dir.close().unwrap();
}
