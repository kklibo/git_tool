use std::path::PathBuf;
use std::process::Command;

pub struct Runner {
    pub dir: PathBuf,
}

impl Runner {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
    /// Run a command and assert success.
    pub fn command(&self, bin: &str, args: &[&str]) {
        assert!(Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap()
            .status
            .success());
    }

    #[allow(dead_code)]
    pub fn command_dbg(&self, bin: &str, args: &[&str]) {
        assert!(dbg!(Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap())
        .status
        .success());
    }

    pub fn stdout(&self, bin: &str, args: &[&str]) -> String {
        let a = Command::new(bin)
            .args(args)
            .current_dir(&self.dir)
            .output()
            .unwrap();
        String::from_utf8(a.stdout).unwrap()
    }
}
