use clap::Parser;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    in_file: PathBuf,
    #[clap(parse(from_os_str))]
    out_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = match std::fs::read_to_string(&args.in_file) {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };
    let mut file = create_out_file(args)?;

    // let display = out_file.display();
    // println!("file content: {}", content);
    // println!("file content size: {}", content.lines().count());
    // println!("file path: {:?}", out_file);
    // println!("file display: {}", display);
    write_content(&mut file, content)?;
    file.sync_data()?;
    Ok(())
}

fn write_content(file: &mut File, content: String) -> std::io::Result<()> {
    for line in content.lines() {
        let text = line.replace("\\n", "\n");
        writeln!(file, "{}", text)?;
    }
    Ok(())
}

fn create_out_file(args: Cli) -> std::io::Result<File> {
    let path = match args.out_file {
        Some(name) => PathBuf::from(name),
        None => {
            let mut path = PathBuf::from(&args.in_file);
            args.in_file.file_stem().and_then(|file_name| {
                let new_name = format!("{} - copy", file_name.to_str().unwrap_or(""));
                path.set_file_name(new_name);
                args.in_file.extension().and_then(|extension| {
                    path.set_extension(extension);
                    Some(())
                });
                Some(())
            });
            path
        }
    };
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
