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
fn parse_git_log(s: &str) -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    for line in s.split('\n') {
        let mut tokens = line.split(' ');
        let hash = tokens.next().unwrap().to_string();
        let message = tokens.next().unwrap().to_string();
        assert!(matches!(tokens.next(), None));
        assert!(matches!(hash_map.insert(message, hash), None));
    }
    hash_map
}

#[test]
fn f() {
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 5);

    println!(
        "{}",
        in_repo_dir.stdout("git", &["log", "--pretty=oneline"])
    );
    println!(
        "{}",
        in_repo_dir.stdout("git", &["log", "--pretty=format:%H %s"])
    );
    println!("{}", in_repo_dir.stdout("git", &["status"]));

    let log_output = in_repo_dir.stdout("git", &["log", "--pretty=format:%H %s"]);
    let commits = parse_git_log(&log_output);

    dbg!(commits);

    temp_dir.close().unwrap();
}
