//! Tests command line invocation of git.

use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

struct Runner<P: AsRef<Path>> {
    dir: P,
}
impl<P> Runner<P>
where
    P: AsRef<Path>,
{
    fn new(dir: P) -> Self {
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

#[test]
fn f() {
    const REPO_NAME: &str = "test_repo";
    let temp_dir = tempdir().unwrap();
    let repo_dir_path = temp_dir.path().join(REPO_NAME);

    let in_temp_dir = Runner::new(&temp_dir);
    let in_repo_dir = Runner::new(&repo_dir_path);

    in_temp_dir.command_dbg("which", &["git"]);
    in_temp_dir.command_dbg("git", &["init", REPO_NAME]);

    in_repo_dir.command_dbg("git", &["status"]);
    in_repo_dir.command_dbg("git", &["config", "user.email", "test@test"]);
    in_repo_dir.command_dbg("git", &["config", "user.name", "test"]);
    in_repo_dir.command_dbg("git", &["commit", "-m", "1", "--allow-empty"]);
    in_repo_dir.command_dbg("git", &["commit", "-m", "2", "--allow-empty"]);

    println!("{}", in_repo_dir.stdout("git", &["log", "--oneline"]));
    println!("{}", in_repo_dir.stdout("git", &["status"]));

    temp_dir.close().unwrap();
}
