use clap::{Arg, Command};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let matches = Command::new("md_cat")
        .version("1.0")
        .author("Gilles Biagomba <gilles.infosec@gmail.com>")
        .about("Concatenates Markdown files from a directory into a single file")
        .arg(
            Arg::new("directory")
                .help("Directory to search for .md files")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .help("Output file to write the concatenated markdown content")
                .required(true),
        )
        .get_matches();

    let dir_path = matches.get_one::<String>("directory").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    process_directory(Path::new(dir_path), output_file)?;

    println!("✅ Markdown content has been written to {output_path}");
    Ok(())
}

fn process_directory(path: &Path, mut output_file: File) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_directory(&path, output_file.try_clone()?)?;
        } else if let Some(ext) = path.extension() {
            if ext == "md" {
                let content = fs::read_to_string(&path)?;
                writeln!(
                    output_file,
                    "\n\n---\n# File: {}\n---\n\n{}",
                    path.display(),
                    content
                )?;
            }
        }
    }
    Ok(())
}