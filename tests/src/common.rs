//! Common test code

use super::runner::Runner;
use std::collections::HashMap;
use tempfile::TempDir;

/// Command-line arguments from macro inputs.
#[macro_export]
macro_rules! args {
    ($($e:expr),+ $(,)?) => {{
        let mut v = vec![];
        $(v.append(&mut shlex::split($e).unwrap());)+
        v
    }};
}

/// Sets up a git repo for testing; returns a Runner targeting the repo dir.
pub fn set_up_repo(temp_dir: &TempDir) -> Runner {
    const REPO_NAME: &str = "test_repo";
    let repo_dir_path = temp_dir.path().join(REPO_NAME);

    let in_temp_dir = Runner::new(temp_dir.path().to_path_buf());
    let in_repo_dir = Runner::new(repo_dir_path);

    in_temp_dir.command(&args!("which git"));
    in_temp_dir.command(&args!("git init", REPO_NAME));
    in_repo_dir.command(&args!("git status"));
    in_repo_dir.command(&args!("git config user.email test@test"));
    in_repo_dir.command(&args!("git config user.name test"));

    in_repo_dir
}

fn commit_message(i: usize) -> String {
    format!("commit{}", i)
}

pub fn do_commits(runner: &Runner, count: usize) {
    for i in 1..=count {
        let message = commit_message(i);
        runner.command(&args!("git commit -m", &message, "--allow-empty"));
    }
}

/// Parses the output of `git log --pretty=format:%H %s`:
/// output lines look like
///
/// 877d0d9433308ae754eb6bc02c402598994c9ef0 commit_message
///
/// Returns a Hashmap of commit_message -> hash.
pub fn parse_git_log(log: &str) -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    for line in log.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split(' ');
        let hash = tokens.next().unwrap().to_string();
        let message = tokens.next().unwrap().to_string();
        assert!(matches!(tokens.next(), None));
        assert!(matches!(hash_map.insert(message, hash), None));
    }
    hash_map
}

/// Parses the output of `git log --pretty=format:%s`
/// (each line is just a commit message) and checks that
/// commit messages from `fn commit_message` generated with
/// `commit_order`'s values match the log.
pub fn match_git_log(log: &str, commit_order: &[usize]) -> bool {
    let lines = log
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if lines.len() != commit_order.len() {
        return false;
    }
    for (&line, &num) in lines.iter().zip(commit_order.iter()) {
        if line != commit_message(num) {
            return false;
        }
    }
    true
}

pub fn match_branch_history(runner: &Runner, branch_name: &str, commit_order: &[usize]) -> bool {
    let log = runner.stdout(&args!("git log", branch_name, "--pretty=format:%s"));
    match_git_log(&log, commit_order)
}

#[test]
fn test_parse_git_log() {
    assert!(parse_git_log("").is_empty());

    let log = r#"
0a07879c0e13ef54f93a0eec2c377d3d3bbe9641 commit5
61dbfae34e916fb710117c4c3064882dcfdc6701 commit4

71511da451154936a64e0e881d3f680f6d056b44 commit3
27ed74c41300b1df1009b66c315120b0d53085e6 commit2
59b9b1f2cd435f5d5cbc32da91c60a159591886b commit1"#;
    let hash_map = parse_git_log(log);
    assert_eq!(hash_map.len(), 5);
}

#[test]
fn test_match_git_log() {
    assert!(match_git_log("", &[]));
    assert!(!match_git_log("", &[1]));
    assert!(!match_git_log("commit1", &[]));
    assert!(!match_git_log("commit1", &[2]));
    assert!(match_git_log("commit1", &[1]));

    let log = r#"
commit5
commit4

commit3
commit2
commit1"#;
    assert!(match_git_log(log, &[5, 4, 3, 2, 1]));
    assert!(!match_git_log(log, &[4, 3, 2, 1]));
    assert!(!match_git_log(log, &[6, 5, 4, 3, 2, 1]));
    assert!(!match_git_log(log, &[5, 4, 3, 2, 6]));
}
