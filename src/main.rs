use args::Args;
use clap::Parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::process::ExitCode;

mod args;

// We can't just use Result<(), Box<dyn Error>> because that wouldn't let us return a non-zero exit code with no error message attached to it
fn main() -> ExitCode {
    let Args { path, mut strings } = Args::parse();
    if let Some(parent_dir) = path.clone().parent() {
        if let Err(error) = fs::create_dir_all(parent_dir) {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
    }
    let mut file = match OpenOptions::new().read(true).write(true).create(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
    };
    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        eprintln!("{}", error);
        return ExitCode::FAILURE;
    };
    let mut contents: Vec<String> = contents.trim_end().lines().map(|line| line.to_owned()).collect();
    strings.retain(|string| !contents.contains(string));
    let lines = contents.len();
    contents.extend(strings);
    if contents.len() != lines {
        if let Err(error) = file.rewind() {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        };
        if let Err(error) = write!(file, "{}", contents.join("\n")) {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }
    ExitCode::FAILURE
}
