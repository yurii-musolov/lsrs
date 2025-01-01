mod cli;
mod constant;
mod util;

use constant::{EXIT_STATUS_MINOR_PROBLEM, EXIT_STATUS_OK, EXIT_STATUS_SERIOUS_TROUBLE};
use regex::Regex;
use std::{
    fs::read_dir,
    io::Error,
    path::{Path, PathBuf},
    process::exit,
};
use util::{by, every, temporary};

fn main() {
    let args = cli::read_args();

    let ignore_temp = temporary(args.ignore_backups);
    let no_hide = by(make_regex(args.hide_pattern), true);
    let show = by(make_regex(args.show_pattern), false);
    let rules: Vec<&dyn Fn(&str) -> bool> = vec![&ignore_temp, &no_hide, &show];
    let check_by_rules = every(&rules);

    let handler = |is_dir: bool, path: &str| {
        if !check_by_rules(path) {
            return;
        }

        if is_dir {
            println!("\x1b[93m{path}\x1b[0m");
        } else {
            println!("{path}");
        }
    };

    let base_path_buf = args.path.unwrap_or(PathBuf::from("."));
    let base_path_buf = match base_path_buf.as_path().canonicalize() {
        Ok(path_buf) => path_buf,
        Err(err) => {
            eprintln!("{err}");
            exit(EXIT_STATUS_SERIOUS_TROUBLE);
        }
    };
    let base_path = base_path_buf.as_path();

    let mut stack = Vec::new();

    match read_dir(base_path) {
        Ok(dir_reader) => stack.push(dir_reader),
        Err(err) => error_handler(err),
    };

    while !stack.is_empty() {
        let dir_entry = stack.last_mut().unwrap();
        match dir_entry.next() {
            Some(Ok(dir_entry)) => {
                let path = dir_entry.path();
                let is_dir = path.is_dir();
                let path_display = path.as_path();

                handler(is_dir, relative(path_display, base_path));

                if args.recursive && is_dir {
                    match read_dir(path_display) {
                        Ok(dir_reader) => stack.push(dir_reader),
                        Err(err) => error_handler(err),
                    };
                }
            }
            Some(Err(err)) => error_handler(err),
            None => {
                let _ = stack.pop();
            }
        };
    }

    exit(EXIT_STATUS_OK)
}

pub fn make_regex(pattern: Option<String>) -> Option<Regex> {
    match pattern {
        Some(pattern) => match Regex::new(&pattern) {
            Ok(re) => Some(re),
            Err(err) => {
                eprintln!("Regex creation failed. Pattern: {pattern} Error: {err}");
                exit(EXIT_STATUS_SERIOUS_TROUBLE);
            }
        },
        None => None,
    }
}

fn error_handler(err: Error) {
    eprintln!("{err}");
    exit(EXIT_STATUS_MINOR_PROBLEM);
}

fn relative<'a>(child: &'a Path, base: &'a Path) -> &'a str {
    child.strip_prefix(base).unwrap().to_str().unwrap()
}
