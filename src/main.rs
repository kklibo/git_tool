use clap::Parser;
use std::process::{Command, Output};

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

fn main() {
    let args = Args::parse();
    dbg!(&args);

    run("which", &["git"]);
    git!("branch parent", &args.parent_hash);
    git!("branch section", &args.section_hash);

    git!("checkout parent");
    git!("merge --squash --no-commit section");
    git!("commit -m", &args.commit_message, "--allow-empty");

    let diff = git!("diff parent section");
    assert!(diff.stdout.is_empty());

    git!("rebase --onto parent section master");
}
