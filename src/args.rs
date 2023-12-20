use clap::Parser;
use std::path::PathBuf;

/// Add strings to a file on their own lines, if they aren't already there.
/// A write to the file is only done if at least one of the specified strings need to be added.
/// Final newlines are trimmed in that case.
/// The file (along with its parent directories) is created if it doesn't exist.
/// If all of the specified strings are already in the file, a non-zero exitcode is returned with no error message.
#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    pub path: PathBuf,
    pub strings: Vec<String>,
}
