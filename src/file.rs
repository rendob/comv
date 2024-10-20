use std::{
    fs, io,
    path::{Path, PathBuf},
};

use mime_guess::{self, mime};

const OUTPUT_DIR_SUFFIX: &str = "_dest";

pub fn is_video(input_file_path: &Path) -> bool {
    let guess = mime_guess::from_path(input_file_path);
    guess
        .first()
        .is_some_and(|guessed| guessed.type_() == mime::VIDEO)
}

pub fn get_output_dir_path(input_dir_path: &Path, is_in_input_dir: bool) -> PathBuf {
    if is_in_input_dir {
        return input_dir_path.join(OUTPUT_DIR_SUFFIX);
    }

    let input_dir_name = input_dir_path.file_name().unwrap_or_default();
    let output_dir_name = input_dir_name.to_string_lossy().into_owned() + OUTPUT_DIR_SUFFIX;
    let output_dir_path = input_dir_path.with_file_name(output_dir_name);
    output_dir_path
}

pub fn get_files<P: AsRef<Path>>(dir_path: P, is_recursive: bool) -> io::Result<Vec<PathBuf>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::mp4("a.mp4", true)]
    #[case::MP4("a.MP4", true)]
    #[case::mov("a.mov", true)]
    #[case::MOV("a.MOV", true)]
    #[case::txt("a.txt", false)]
    #[case::m4a("a.m4a", false)]
    #[case::png("a.png", false)]
    #[case::gif("a.gif", false)]
    #[case::DS_Store(".DS_Store", false)]
    #[allow(non_snake_case)]
    fn test_is_video(#[case] filename: &str, #[case] expected: bool) {
        let sut = Path::new(filename);

        let result = is_video(sut);

        assert_eq!(result, expected);
    }
}
