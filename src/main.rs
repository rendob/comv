use std::{error::Error, fs};

mod args;
mod compress;
mod file;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();
    let input_dir_path = fs::canonicalize(args.input_dir)?;

    let input_file_paths = file::get_files(&input_dir_path, args.recursive)?;
    let output_dir_path = file::get_output_dir_path(&input_dir_path, !args.recursive);

    for input_file_path in input_file_paths {
        compress::compress_file(&input_file_path, &input_dir_path, &output_dir_path)?;
    }

    Ok(())
}
