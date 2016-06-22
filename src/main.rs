extern crate chrono;

use std::fmt;
use std::process::Command;
use std::str;
use std::iter;
// use chrono::datetime::DateTime;
// use chrono::Local;
// use chrono::offset::TimeZone;
use chrono::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Branch {
    user: String,
    date: DateTime<FixedOffset>,
    branch: String,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "[{}] {} {}",
               self.date.format("%Y-%m-%d"),
               self.user,
               self.branch)
    }
}

impl fmt::Debug for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "user: {}\nbranch: {}\ndate: {}",
               self.user,
               self.branch,
               self.date)
    }
}

fn parse_repo(line: String) -> Option<Branch> {
    if !line.contains("origin/HEAD") && !line.contains("origin/master") {
        let mut user = String::new();
        let mut date = String::new();
        let mut branch = String::new();
        for (i, part) in line.split("^").enumerate() {
            match i {
                0 => user = part.to_string(),
                1 => branch = part.to_string().replace("origin/", ""),
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
    let mut v = get_git_data();
    v.sort();
    print_git_data(v);
}

fn print_git_data(v: Vec<Branch>) {
    let max = v.iter().fold(0, |max, branch| {
        let len = branch.user.len();
        if len > max {
            len
        } else {
            max
        }
    });
    println!("max: {}", max);
    for branch in v {
        let padding_count = max - branch.branch.len();
        let padding: String = iter::repeat(" ").take(padding_count).collect();
        print!("[{}]", branch.date.format("%Y-%m-%d"));
        print!("{}", branch.branch);
        println!("{}", padding);
    }
}

fn get_git_data() -> Vec<Branch> {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(authorname)^%(refname:short)^%(committerdate:rfc2822)")
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
