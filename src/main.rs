use clap::Parser;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> = std::result::Result<T, E>;

#[derive(Debug)]
struct HumanError(String);

impl HumanError {
    fn new(error: String) -> Self {
        Self(error)
    }
}

impl fmt::Display for HumanError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::error::Error for HumanError {}

/// Replaces \n with newlines
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Input file
    #[clap(parse(from_os_str))]
    in_file: PathBuf,
    /// Output file
    #[clap(short, long, parse(from_os_str))]
    out_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match run(&args) {
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
        _ => std::process::exit(0),
    }
}

fn run(args: &Args) -> Result<()> {
    let content = std::fs::read_to_string(&args.in_file).map_err(|err| {
        let error = format!("Error when reading '{}': {}", args.in_file.display(), err);
        HumanError::new(error)
    })?;
    let out_path = out_path(args);
    let mut file = create_out_file(&out_path).map_err(|err| {
        let error = format!("Error when creating file '{}': {}", out_path.display(), err);
        HumanError::new(error)
    })?;
    write_content(&mut file, content).map_err(|err| {
        let error = format!("Writing error: {}", err);
        HumanError::new(error)
    })?;
    file.sync_data().map_err(|err| {
        let error = format!("Cannot sync data with disk: {}", err);
        HumanError::new(error)
    })?;
    Ok(())
}

fn write_content(file: &mut File, content: String) -> Result<(), std::io::Error> {
    for line in content.lines() {
        let text = line.replace("\\n", "\n");
        writeln!(file, "{}", text)?;
    }
    Ok(())
}

fn out_path(args: &Args) -> PathBuf {
    match &args.out_file {
        Some(name) => PathBuf::from(name),
        None => {
            let mut path = PathBuf::from(&args.in_file);
            args.in_file.file_stem().map(|file_name| {
                let new_name = format!("{} - copy", file_name.to_str().unwrap_or(""));
                path.set_file_name(new_name);
                args.in_file
                    .extension()
                    .map(|extension| path.set_extension(extension))
            });
            path
        }
    }
}

fn create_out_file(path: &PathBuf) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

#[test]
fn check_path_file_name() {
    let mut name = PathBuf::from("../some.txt");
    name.set_file_name("some copy.txt");
    assert_eq!(name, PathBuf::from("../some copy.txt"));
}
