extern crate chrono;

use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs, process::Command};

fn main() {
    let mut out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    for _ in 0..10 {
        out_dir = out_dir.join("../");
        let res = read_dir_file(&out_dir.join(PathBuf::from_str(".git/logs").unwrap()));
        for p in &res {
            println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
        }
        if res.len() != 0 {
            break;
        }
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("git-commit-id.txt"), commit_id())
        .expect("Write git-commit-id.txt failed.");

    fs::write(out_dir.join("git-commit-date.txt"), commit_date())
        .expect("Write git-commit-date.txt failed.");

    fs::write(
        out_dir.join("build_date.txt"),
        chrono::Local::now().to_string(),
    )
    .expect("Write build_date.txt failed.");

    fs::write(out_dir.join("branch.txt"), git_branch()).expect("Write git-commit-date.txt failed.");
}

fn commit_id() -> String {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    std::env::set_current_dir(out_dir).unwrap();

    let output = Command::new("git")
        .args(vec!["rev-parse", "HEAD"])
        .output()
        .expect("Unable to get git commit id");
    ::std::str::from_utf8(&output.stdout)
        .unwrap_or("Unknown")
        .trim()
        .to_owned()
}

fn commit_date() -> String {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    std::env::set_current_dir(out_dir).unwrap();

    let output = Command::new("git")
        .args(&[
            "log",
            "-1",
            "--date=format:%Y-%m-%d %H:%M:%S",
            "--pretty=format:%cd",
        ])
        .output()
        .expect("Unable to get git commit date");
    ::std::str::from_utf8(&output.stdout)
        .unwrap_or("Unknown")
        .trim()
        .to_owned()
}

fn git_branch() -> String {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    std::env::set_current_dir(out_dir).unwrap();

    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Unable to get git commit date");
    ::std::str::from_utf8(&output.stdout)
        .unwrap_or("Unknown")
        .trim()
        .to_owned()
}

fn read_dir_file(path: &PathBuf) -> Vec<PathBuf> {
    let dir = match std::fs::read_dir(path) {
        Ok(x) => x,
        Err(_) => return vec![],
    };
    dir.map(|inner| {
        let mut res = vec![];
        if let Ok(inner) = inner {
            let inner_path = inner.path();
            if path == &inner_path {
                return vec![];
            }
            if inner_path.is_file() {
                res.push(inner_path);
            } else if path.is_dir() {
                res.append(&mut read_dir_file(&inner_path));
            }
        }
        res
    })
    .collect::<Vec<_>>()
    .concat()
}
