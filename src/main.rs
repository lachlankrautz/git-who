extern crate chrono;

use std::fmt;
use std::process::Command;
use std::str;
use chrono::date::Date;
use chrono::Local;
use chrono::offset::TimeZone;

struct Branch {
    user: String,
    date: Date<Local>,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "user: {}\ndate: {}", self.user, self.date)
    }
}

fn parse_repo(line: String) -> Option<Branch> {
    if line.contains("refs/remotes/origin/") && line != "refs/remotes/origin/HEAD" &&
       line != "refs/remotes/origin/master" {
        Some(Branch {
            user: line,
            date: Local.ymd(2016, 2, 1),
        })
    } else {
        None
    }
}

fn main() {

    let v = get_git_data();

    for branch in v {
        println!("branch: {}", branch);
    }
}

fn get_git_data() -> Vec<Branch> {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(authorname),%(refname),%(committerdate)")
        .output()
        .expect("failed to execute process");
    let git_output = str::from_utf8(&output.stdout).unwrap();

    let mut v: Vec<Branch> = Vec::new();
    for line in git_output.lines() {
        parse_repo(line.to_string()).map(|x| v.push(x));
    }
    v
}
