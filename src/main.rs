use args::Args;
use clap::Parser;
use std::fs;
use std::process::ExitCode;

mod args;

fn main() -> ExitCode {
    let Args { path, mut strings } = Args::parse();
    let contents = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
    };
    let mut contents: Vec<String> = contents.trim_end().lines().map(|line| line.to_owned()).collect();
    strings.retain(|string| !contents.contains(string));
    let lines = contents.len();
    contents.extend(strings);
    if contents.len() != lines {
        if let Err(error) = fs::write(path, contents.join("\n")) {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }
    ExitCode::FAILURE
}
