use clap::Parser;
use log::{error, info, LevelFilter, Log, Metadata, Record};
use std::process::{Command, Output};
use std::sync::Mutex;

#[derive(Parser, Debug)]
struct Args {
    parent_hash: String,
    section_hash: String,
    commit_message: String,
}

/// Runs `git` with space-separated arguments from macro inputs.
macro_rules! git {
    ($($e:expr),+ $(,)?) => {
        {
            let mut v = vec![];
            $(v.extend($e.split(" "));)+
            run("git", &v)
        }
    };
}

pub fn run(bin: &str, args: &[&str]) -> Output {
    let output = Command::new(bin).args(args).output().unwrap();
    if !output.status.success() {
        panic!("non-zero exit status from: {} {}", bin, args.join(" "));
    }
    output
}

struct StringLogger {
    s: Mutex<String>,
}
impl StringLogger {
    const fn new() -> Self {
        Self {
            s: Mutex::new(String::new()),
        }
    }
    fn get(&self) -> String {
        let a = self.s.lock().unwrap();
        a.clone()
    }
}
impl Log for StringLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let mut a = self.s.lock().unwrap();
        a.push_str(&format!("{} - {}\n", record.level(), record.args()));
    }

    fn flush(&self) {}
}

static LOGGER: StringLogger = StringLogger::new();
fn main() {
    let args = Args::parse();

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    info!("test");
    error!("test");

    run("which", &["git"]);
    let target_branch = git!("branch --show-current");
    let target_branch = String::from_utf8(target_branch.stdout).unwrap();
    let target_branch = target_branch.trim();
    git!("branch parent", &args.parent_hash);
    git!("branch section", &args.section_hash);

    git!("checkout parent");
    git!("merge --squash --no-commit section");
    git!("commit -m", &args.commit_message, "--allow-empty");

    let diff = git!("diff parent section");
    assert!(diff.stdout.is_empty());

    git!("rebase --onto parent section", target_branch);

    let parent_short_hash = git!("show parent --format=%h --no-patch");
    let parent_short_hash = String::from_utf8(parent_short_hash.stdout).unwrap();
    let tag_name = format!("archive/{}", parent_short_hash.trim());
    git!("tag", &tag_name, "section");

    git!("checkout", target_branch);
    git!("branch --delete --force parent section");

    print!("{}", LOGGER.get());
}
