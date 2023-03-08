//! Tests command line invocation of git.

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

#[test]
fn f() {
    let temp_dir = tempdir().unwrap();

    let in_repo_dir = set_up_repo(&temp_dir);
    do_commits(&in_repo_dir, 2);

    println!(
        "{}",
        in_repo_dir.stdout("git", &["log", "--pretty=format:%s"])
    );
    println!("{}", in_repo_dir.stdout("git", &["status"]));

    temp_dir.close().unwrap();
}
