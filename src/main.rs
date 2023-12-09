use args::Args;
use clap::Parser;
use std::error::Error;
use std::fs;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { path, mut strings } = Args::parse();
    let mut contents: Vec<String> = fs::read_to_string(&path)?
        .trim_end()
        .lines()
        .map(|line| line.to_owned())
        .collect();
    strings.retain(|string| !contents.contains(string));
    contents.extend(strings);
    fs::write(path, contents.join("\n"))?;
    Ok(())
}
