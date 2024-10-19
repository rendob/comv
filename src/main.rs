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

fn get_output_dir_path(input_dir_path: &Path) -> PathBuf {
    let input_file_name = input_dir_path.file_name().unwrap_or_default();
    let output_dir_name = input_file_name.to_string_lossy().into_owned() + "_compressed";
    let output_dir_path = input_dir_path.with_file_name(output_dir_name);
    output_dir_path
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

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let input_dir_path = Path::new(&args.input_dir);

    let output_dir_path = get_output_dir_path(input_dir_path);
    println!("{:?}", output_dir_path);

    let file_paths = get_files_recursively(input_dir_path)?;
    println!("{:#?}", file_paths);

    Ok(())
}
