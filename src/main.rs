use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    parent_hash: String,
    section_hash: String,
    commit_message: String,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
