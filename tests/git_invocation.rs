//! Tests command line invocation of git.

use std::process::Command;
use tempfile::tempdir;

#[test]
fn f() {
    let dir = tempdir().unwrap();

    assert!(Command::new("which")
        .arg("git")
        .output()
        .unwrap()
        .status
        .success());
    assert!(Command::new("git")
        .arg("init")
        .arg("test_repo")
        .output()
        .unwrap()
        .status
        .success());

    let a = Command::new("git").arg("status").output().unwrap();
    dbg!(a);

    dir.close().unwrap();
}
