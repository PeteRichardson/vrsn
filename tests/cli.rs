//! The stamp in the real binary. These tests do not expect a tag or a branch
//! name: CI uses a shallow clone with no tags and a detached `HEAD`.

use std::process::Command;

fn vrsn(arg: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_vrsn"))
        .arg(arg)
        .output()
        .expect("vrsn runs");
    assert!(out.status.success());
    String::from_utf8(out.stdout).expect("UTF-8 output")
}

#[test]
fn short_version_starts_with_the_release_version() {
    let out = vrsn("-V");
    assert!(
        out.starts_with(concat!("vrsn ", env!("CARGO_PKG_VERSION"))),
        "{out}"
    );
    assert_eq!(out.lines().count(), 1, "{out}");
}

#[test]
fn long_version_starts_with_the_short_line_and_shows_the_build_time() {
    let short = vrsn("-V");
    let long = vrsn("--version");
    assert_eq!(long.lines().next(), short.lines().next());
    assert!(long.lines().any(|l| l.starts_with("built: ")), "{long}");
}

#[test]
fn help_shows_the_same_facts_as_long_version() {
    let long = vrsn("--version");
    assert!(vrsn("--help").contains(long.trim_end()));
}
