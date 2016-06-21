extern crate chrono;

use std::fmt;
use std::process::Command;
use std::str;
// use chrono::datetime::DateTime;
// use chrono::Local;
// use chrono::offset::TimeZone;
use chrono::*;

struct Branch {
    user: String,
    branch: String,
    date: DateTime<FixedOffset>,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "user: {}\nbranch: {}\ndate: {}",
               self.user,
               self.branch,
               self.date)
    }
}

fn parse_repo(line: String) -> Option<Branch> {
    if !line.contains("refs/remotes/origin/HEAD") && !line.contains("refs/remotes/origin/master") {

        let mut user = String::new();
        let mut date = String::new();
        let mut branch = String::new();
        for (i, part) in line.split("^").enumerate() {
            match i {
                0 => user = part.to_string(),
                1 => branch = part.to_string(),
                2 => date = part.to_string(),
                _ => {}
            }
        }
        Some(Branch {
            user: user,
            branch: branch,
            date: DateTime::parse_from_rfc2822(&date).unwrap(),
        })
    } else {
        None
    }
}

fn main() {

    let v = get_git_data();

    for branch in v {
        println!("[branch]\n{}", branch);
    }
}

fn get_git_data() -> Vec<Branch> {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--sort=-committerdate")
        .arg("--format=%(authorname)^%(refname)^%(committerdate:rfc2822)")
        .arg("refs/remotes/origin/")
        .output()
        .expect("failed to execute process");
    let git_output = str::from_utf8(&output.stdout).unwrap();

    let mut v: Vec<Branch> = Vec::new();
    for line in git_output.lines() {
        parse_repo(line.to_string()).map(|x| v.push(x));
    }
    v
}
