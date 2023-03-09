use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser, Debug)]
struct Args {
    parent_hash: String,
    //section_hash: String,
    //commit_message: String,
}

pub fn run(bin: &str, args: &[&str]) -> Output {
    Command::new(bin).args(args).output().unwrap()
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    run("which", &["git"]);
    run("git", &["branch", "parent", &args.parent_hash]);
}
