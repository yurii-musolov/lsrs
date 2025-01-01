use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Args {
    pub path: Option<PathBuf>,
    /// do not list implied entries ending with ~
    #[clap(short = 'B', long = "ignore-backups", default_value_t = false)]
    pub ignore_backups: bool,
    /// do not list implied entries matching shell PATTERN
    #[clap(long = "hide")]
    pub hide_pattern: Option<String>,
    /// do list implied entries matching shell PATTERN
    #[clap(long = "show")]
    pub show_pattern: Option<String>,
    /// list subdirectories recursively
    #[clap(short = 'R', long = "recursive", default_value_t = false)]
    pub recursive: bool,
}

pub fn read_args() -> Args {
    Args::parse()
}
