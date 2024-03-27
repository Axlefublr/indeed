use std::path::PathBuf;

use clap::Parser;

/// Appends lines of text to a file on their own lines.
/// A write to the file is only done if at least one of the specified strings
/// needs to be added. Final newlines are trimmed in that case.
/// The file (along with its parent directories) is created if it doesn't exist.
#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    pub path:    PathBuf,
    pub strings: Vec<String>,
    /// Only append a line if it's not already in the file.
    /// This check happens separately for each line.
    /// The matching is exact, not substring.
    /// If all of the specified strings are already in the file, a non-zero
    /// exitcode is returned with no error message.
    #[arg(short, long)]
    pub unique:  bool,
}
