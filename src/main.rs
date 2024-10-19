use std::{
    fs::{self},
    io::Error,
    path::{Path, PathBuf},
};

use clap::Parser;

/// compress videos in a directory
#[derive(Parser)]
struct Args {
    /// path of the directory that contains uncompressed videos
    input_dir: String,
}

fn get_files_recursively<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>, Error> {
    let mut paths: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            paths.push(entry.path());
        } else if file_type.is_dir() {
            let mut children = get_files_recursively(entry.path())?;
            paths.append(&mut children);
        }
    }

    Ok(paths)
}

fn main() {
    let args = Args::parse();

    let output_dir = format!("{}_compressed", &args.input_dir);
    println!("{}", output_dir);

    let file_paths = get_files_recursively(args.input_dir);
    println!("{:#?}", file_paths);
}
