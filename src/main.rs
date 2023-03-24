mod string_logger;

use clap::Parser;
use log::{error, info, LevelFilter};
use std::process::{Command, Output};
use string_logger::StringLogger;

#[derive(Parser, Debug)]
struct Args {
    parent_hash: String,
    section_hash: String,
    commit_message: String,
    #[arg(short, long)]
    /// Show log on success
    verbose: bool,
}

/// Runs `git` with space-separated arguments from macro inputs.
macro_rules! git {
    ($($e:expr),+ $(,)?) => {
        {
            let mut v = vec![];
            $(
                v.append(&mut shlex::split($e).unwrap_or_else(||{
                    error!("while assembling next git command: argument parsing failure: \"{}\"", $e);
                    quit();
                }));
            )+
            let v: Vec<_> = v.iter().map(|s|s.as_str()).collect();
            run("git", &v)
        }
    };
}

fn run(bin: &str, args: &[&str]) -> Output {
    let quoted: Vec<_> = args.iter().map(|s| shlex::quote(s)).collect();
    info!("running: \"{bin} {}\"", quoted.join(" "));

    let output = Command::new(bin).args(args).output().unwrap_or_else(|e| {
        error!("{e} from: \"{bin} {}\"", quoted.join(" "));
        quit();
    });
    if !output.status.success() {
        error!("{} from: \"{bin} {}\"", output.status, quoted.join(" "));
        quit();
    }
    output
}

fn quit() -> ! {
    info!("exiting: exit status 1");
    eprintln!("FAILED, dumping log:");
    eprint!("{}", LOGGER.get());
    std::process::exit(1);
}

static LOGGER: StringLogger = StringLogger::new();
fn main() {
    let args = Args::parse();

    // `unwrap` is fine here: this only fails if `set_logger` has already been called.
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    run("which", &["git"]);
    let target_branch = git!("branch --show-current");
    let target_branch = String::from_utf8(target_branch.stdout).unwrap_or_else(|_| {
        error!("while recording current branch name: invalid UTF-8 in branch name");
        quit();
    });
    let target_branch = target_branch.trim();
    git!("branch parent", &args.parent_hash);
    git!("branch section", &args.section_hash);

    git!("checkout parent");
    git!("merge --squash --no-commit section");
    let commit_message = shlex::quote(&args.commit_message);
    git!("commit -m", &commit_message, "--allow-empty");

    let diff = git!("diff parent section");
    if !diff.stdout.is_empty() {
        error!("squash merge check failed: diff is not empty!");
        quit();
    }

    git!("rebase --onto parent section", target_branch);

    let parent_short_hash = git!("show parent --format=%h --no-patch");
    let parent_short_hash = String::from_utf8(parent_short_hash.stdout).unwrap_or_else(|_| {
        error!("while preparing archive tag name: invalid UTF-8 in hash value");
        quit();
    });
    let tag_name = format!("archive/{}", parent_short_hash.trim());
    git!("tag", &tag_name, "section");

    git!("checkout", target_branch);
    git!("branch --delete --force parent section");

    info!("OK");
    if args.verbose {
        print!("{}", LOGGER.get());
    }
}
