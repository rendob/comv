use std::{
    error,
    fs::{self},
    io,
    path::{Path, PathBuf},
    process::Command,
};

use mime_guess::{self, mime};

mod args;

fn get_output_dir_path(input_dir_path: &Path, is_in_input_dir: bool) -> PathBuf {
    if is_in_input_dir {
        return input_dir_path.join("dest");
    }

    let input_dir_name = input_dir_path.file_name().unwrap_or_default();
    let output_dir_name = input_dir_name.to_string_lossy().into_owned() + "_dest";
    let output_dir_path = input_dir_path.with_file_name(output_dir_name);
    output_dir_path
}

fn get_files<P: AsRef<Path>>(dir_path: P, is_recursive: bool) -> Result<Vec<PathBuf>, io::Error> {
    let mut paths: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            paths.push(entry.path());
        } else if file_type.is_dir() && is_recursive {
            let mut children = get_files(entry.path(), is_recursive)?;
            paths.append(&mut children);
        }
    }

    Ok(paths)
}

fn is_video(input_file_path: &Path) -> bool {
    let guess = mime_guess::from_path(input_file_path);
    guess
        .first()
        .is_some_and(|guessed| guessed.type_() == mime::VIDEO)
}

fn compress_file(
    input_file_path: &Path,
    input_dir_path: &Path,
    output_dir_path: &Path,
) -> Result<(), Box<dyn error::Error>> {
    if input_file_path.ends_with(".DS_Store") {
        return Ok(());
    };

    let output_file_path = output_dir_path.join(input_file_path.strip_prefix(input_dir_path)?);
    if output_file_path.exists() {
        return Ok(());
    }

    if let Some(output_file_parent) = output_file_path.parent() {
        fs::create_dir_all(&output_file_parent)?;
    }

    if is_video(input_file_path) {
        Command::new("ffmpeg")
            .args([
                "-ss",
                "0.3",
                "-i",
                input_file_path.to_str().unwrap_or_default(),
                "-nostdin",
                "-vcodec",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-r",
                "60",
                output_file_path.to_str().unwrap_or_default(),
            ])
            .spawn()?
            .wait()?;

        println!(
            "compressed {:?} to {:?}.",
            input_file_path, output_file_path
        );
    } else {
        fs::copy(input_file_path, &output_file_path)?;
        println!("copied {:?} to {:?}.", input_file_path, output_file_path);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = args::parse_args();
    let input_dir_path = fs::canonicalize(args.input_dir)?;

    let input_file_paths = get_files(&input_dir_path, args.recursive)?;
    let output_dir_path = get_output_dir_path(&input_dir_path, !args.recursive);

    for input_file_path in input_file_paths {
        compress_file(&input_file_path, &input_dir_path, &output_dir_path)?;
    }

    Ok(())
}
