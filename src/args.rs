use clap::Parser;
use std::path::PathBuf;

mod about;

#[derive(Parser)]
#[command(author, version, long_about = about::ABOUT)]
pub struct Args {
    pub path: PathBuf,
    pub strings: Vec<String>,
}
