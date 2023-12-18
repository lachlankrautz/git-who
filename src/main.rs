extern crate chrono;
extern crate clap;
extern crate colored;

use chrono::{DateTime, FixedOffset, Local};
use clap::{App, AppSettings, Arg};
use colored::Colorize;
use std::fmt;
use std::iter;
use std::process::Command;
use std::str;

fn main() {
    let args = App::new("git-who")
        .version("0.1.3")
        .about("List remote branches by author and date of last commit")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("remote")
                .help("Remote name [default: origin]")
                .short("r")
                .long("remote")
                .takes_value(true)
                .value_name("NAME"),
        )
        .get_matches();
    let remote = args.value_of("remote").unwrap_or("origin");

    let mut v = get_git_data(remote);
    v.sort();
    print_git_data(v);
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Branch {
    user: String,
    date: DateTime<FixedOffset>,
    branch: String,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] {} {}",
            self.date.format("%Y-%m-%d"),
            self.user,
            self.branch
        )
    }
}

impl fmt::Debug for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user: {}\nbranch: {}\ndate: {}",
            self.user, self.branch, self.date
        )
    }
}

fn parse_repo(line: String) -> Option<Branch> {
    if line.contains("^origin^") {
        return None;
    }

    let mut user = String::new();
    let mut date = String::new();
    let mut branch = String::new();
    for (i, part) in line.split("^~^").enumerate() {
        match i {
            0 => user = part.to_string(),
            1 => branch = part.to_string().replace("origin/", ""),
            2 => date = part.to_string(),
            _ => {}
        }
    }
    Some(Branch {
        user,
        branch,
        date: DateTime::parse_from_rfc2822(&date).unwrap(),
    })
}

fn print_git_data(v: Vec<Branch>) {
    let max = v
        .iter()
        .fold(0, |max, branch| std::cmp::max(max, branch.user.len()));
    let now = Local::now();

    v.iter().for_each(|branch| {
        let padding_count = max - branch.user.len();
        let padding = iter::repeat(" ").take(padding_count).collect::<String>();

        println!(
            "[{}] {}{} {}",
            coloured_date(now, branch.date),
            branch.user,
            padding,
            branch.branch
        );
    });
}

fn coloured_date(now: DateTime<Local>, date: DateTime<FixedOffset>) -> String {
    let str_date = date.format("%Y-%m-%d").to_string();
    let diff = (now.with_timezone(&date.timezone()) - date).num_days();

    if diff >= 180 {
        str_date.red().to_string()
    } else if diff >= 90 {
        str_date.yellow().to_string()
    } else {
        str_date.green().to_string()
    }
}

fn get_git_data(remote: &str) -> Vec<Branch> {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(authorname)^~^%(refname:short)^~^%(committerdate:rfc2822)")
        .arg(format!("refs/remotes/{}/", remote))
        .output()
        .expect("failed to execute process");
    str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| parse_repo(line.to_string()))
        .collect::<Vec<Branch>>()
}
