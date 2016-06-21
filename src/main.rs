extern crate chrono;

use std::process::Command;
use std::str;
use chrono::datetime::DateTime;

struct Repo {
    user: String,
    date: DateTime,
}

fn parse_repo(line: String) -> Option<Repo> {
    if line.contains("refs/remotes/origin/") && line != "refs/remotes/origin/HEAD" &&
       line != "refs/remotes/origin/master" {
        Some(Repo { user: line });
    } else {
        None;
    }
}

fn main() {

    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(authorname),%(refname),%(committerdate)")
        .output()
        .expect("failed to execute process");
    let git_output = str::from_utf8(&output.stdout).unwrap();

    let mut v: Vec<Repo> = Vec::new();
    for line in git_output.lines() {
        // match parse_repo(line.to_string()) {
        // Some(x) => v.push(x),
        // None => 0,
        // }
        //
        parse_repo(line.to_string()).map(|x| v.push(x));
    }

    for repo in v {
        println!("{}", repo.user);
    }
}
