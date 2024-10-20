use std::{
    collections::HashSet,
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

pub fn get_files<P: AsRef<Path>>(dir_path: P, is_recursive: bool) -> io::Result<HashSet<PathBuf>> {
    let mut paths: HashSet<PathBuf> = HashSet::new();

    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            paths.insert(entry.path());
        } else if file_type.is_dir() && is_recursive {
            let children = get_files(entry.path(), is_recursive)?;
            paths.extend(children);
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::File;
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

    #[rstest]
    #[case("/foo/bar", false, "/foo/bar_dest")]
    #[case("/foo/bar", true, "/foo/bar/_dest")]
    #[case("/aaa/bbb/ccc/", false, "/aaa/bbb/ccc_dest")]
    #[case("/aaa/bbb/ccc/", true, "/aaa/bbb/ccc/_dest")]
    fn test_get_output_dir_path(
        #[case] input_dir_path: &str,
        #[case] is_input_dir: bool,
        #[case] expected: &str,
    ) {
        let input_dir_path = Path::new(input_dir_path);
        let result = get_output_dir_path(input_dir_path, is_input_dir);

        let expected = PathBuf::from(expected);
        assert_eq!(result, expected);
    }

    struct TmpDir {
        path: PathBuf,
    }
    impl Drop for TmpDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[rstest]
    #[case::recursive(true, vec![".DS_Store", "foo.mp4", "dir/xxx.png"])]
    #[case(false, vec![".DS_Store", "foo.mp4"])]
    fn test_get_files(#[case] is_recursive: bool, #[case] expected: Vec<&str>) {
        let tmp_dir = TmpDir {
            path: PathBuf::from(format!(".tmp_{is_recursive}")),
        };

        fs::create_dir_all(tmp_dir.path.join("dir/empty")).unwrap();
        let file_paths: HashSet<PathBuf> = [".DS_Store", "foo.mp4", "dir/xxx.png"]
            .into_iter()
            .map(|file_path| tmp_dir.path.join(file_path))
            .collect();
        file_paths.iter().for_each(|file_path| {
            File::create(file_path).unwrap();
        });

        let result = get_files(&tmp_dir.path, is_recursive).unwrap();

        let expected = expected
            .into_iter()
            .map(|file_path| tmp_dir.path.join(file_path))
            .collect();
        assert_eq!(result, expected);
    }
}
