use std::{
    error,
    fs::{self},
    io,
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

fn get_files_recursively<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>, io::Error> {
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

fn compress_file(
    input_file_path: &Path,
    input_dir_path: &Path,
    output_dir_path: &Path,
) -> Result<(), Box<dyn error::Error>> {
    // duplicate directory
    if let Some(input_file_parent) = input_file_path.parent() {
        let output_file_path =
            output_dir_path.join(input_file_parent.strip_prefix(input_dir_path)?);
        fs::create_dir_all(&output_file_path)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let input_dir_path = fs::canonicalize(args.input_dir)?;

    let output_dir_path = get_output_dir_path(&input_dir_path);
    println!("{:?}", output_dir_path);

    let input_file_paths = get_files_recursively(&input_dir_path)?;
    println!("{:#?}", input_file_paths);

    for input_file_path in input_file_paths {
        compress_file(&input_file_path, &input_dir_path, &output_dir_path)?;
    }

    Ok(())
}
