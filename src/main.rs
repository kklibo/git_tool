use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser, Debug)]
struct Args {
    parent_hash: String,
    section_hash: String,
    commit_message: String,
}

pub fn run(bin: &str, args: &[&str]) -> Output {
    Command::new(bin).args(args).output().unwrap()
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    run("which", &["git"]);
    run("git", &["branch", "parent", &args.parent_hash]);
    run("git", &["branch", "section", &args.section_hash]);

    run("git", &["checkout", "parent"]);
    run("git", &["merge", "--squash", "--no-commit", "section"]);
    run(
        "git",
        &["commit", "-m", &args.commit_message, "--allow-empty"],
    );

    let diff = run("git", &["diff", "parent", "section"]);
    assert!(diff.stdout.is_empty());

    run("git", &["rebase", "--onto", "parent", "section", "master"]);
}
