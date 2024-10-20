use std::{error::Error, fs, path::Path, process::Command};

use crate::file;

pub fn compress_file(
    input_file_path: &Path,
    input_dir_path: &Path,
    output_dir_path: &Path,
) -> Result<(), Box<dyn Error>> {
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

    if file::is_video(input_file_path) {
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
