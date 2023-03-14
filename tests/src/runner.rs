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
    /// `args` contains the command to run, followed by 0 or more arguments.
    pub fn command(&self, args: &Vec<String>) {
        assert!(Command::new(args.first().unwrap())
            .args(args.iter().skip(1))
            .current_dir(&self.dir)
            .output()
            .unwrap()
            .status
            .success());
    }

    #[allow(dead_code)]
    pub fn command_dbg(&self, args: &Vec<String>) {
        assert!(dbg!(Command::new(args.first().unwrap())
            .args(args.iter().skip(1))
            .current_dir(&self.dir)
            .output()
            .unwrap())
        .status
        .success());
    }

    pub fn stdout(&self, args: &Vec<String>) -> String {
        let a = Command::new(args.first().unwrap())
            .args(args.iter().skip(1))
            .current_dir(&self.dir)
            .output()
            .unwrap();
        String::from_utf8(a.stdout).unwrap()
    }
}
