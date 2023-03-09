//! Tests command line invocation of git.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{tempdir, TempDir};

struct Runner {
    dir: PathBuf,
}

impl Runner {
    fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
    /// Run a command and assert success.
    fn command(&self, bin: &str, args: &[&str]) {
        assert!(Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap()
            .status
            .success());
    }

    #[allow(dead_code)]
    fn command_dbg(&self, bin: &str, args: &[&str]) {
        assert!(dbg!(Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap())
        .status
        .success());
    }

    fn stdout(&self, bin: &str, args: &[&str]) -> String {
        let a = Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap();
        String::from_utf8(a.stdout).unwrap()
    }
}

/// Sets up a git repo for testing; returns a Runner targeting the repo dir.
fn set_up_repo(temp_dir: &TempDir) -> Runner {
    const REPO_NAME: &str = "test_repo";
    let repo_dir_path = temp_dir.path().join(REPO_NAME);

    let in_temp_dir = Runner::new(temp_dir.path().to_path_buf());
    let in_repo_dir = Runner::new(repo_dir_path);

    in_temp_dir.command("which", &["git"]);
    in_temp_dir.command("git", &["init", REPO_NAME]);
    in_repo_dir.command("git", &["status"]);
    in_repo_dir.command("git", &["config", "user.email", "test@test"]);
    in_repo_dir.command("git", &["config", "user.name", "test"]);

    in_repo_dir
}

fn commit_message(i: usize) -> String {
    format!("commit{}", i)
}

fn do_commits(runner: &Runner, count: usize) {
    for i in 1..=count {
        runner.command(
            "git",
            &["commit", "-m", &commit_message(i), "--allow-empty"],
        );
    }
}

/// Parses the output of `git log --pretty=format:%H %s`:
/// output lines look like
///
/// 877d0d9433308ae754eb6bc02c402598994c9ef0 commit_message
///
/// Returns a Hashmap of commit_message -> hash.
fn parse_git_log(log: &str) -> HashMap<String, String> {
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
fn match_git_log(log: &str, commit_order: &[usize]) -> bool {
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

#[test]
fn test_history_subset_squash() {
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 5);

    let log_output = in_repo_dir.stdout("git", &["log", "--pretty=format:%H %s"]);
    let commits = parse_git_log(&log_output);

    let parent_hash = commits.get("commit2").unwrap();
    let section_hash = commits.get("commit4").unwrap();

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
    in_repo_dir.command("git", &["commit", "-m", "commit6", "--allow-empty"]);
    assert!(in_repo_dir
        .stdout("git", &["diff", "parent", "section"])
        .is_empty());

    in_repo_dir.command("git", &["rebase", "--onto", "parent", "section", "master"]);

    let master_log = in_repo_dir.stdout("git", &["log", "master", "--pretty=format:%s"]);
    let parent_log = in_repo_dir.stdout("git", &["log", "parent", "--pretty=format:%s"]);
    let section_log = in_repo_dir.stdout("git", &["log", "section", "--pretty=format:%s"]);

    assert!(match_git_log(&master_log, &[5, 6, 2, 1]));
    assert!(match_git_log(&parent_log, &[6, 2, 1]));
    assert!(match_git_log(&section_log, &[4, 3, 2, 1]));

    temp_dir.close().unwrap();
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
