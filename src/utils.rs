use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(
    src: U,
    dst: V,
) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(src.as_ref()));

    let output_root = PathBuf::from(dst.as_ref());
    let input_root = PathBuf::from(src.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };

        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;

            if entry.file_type()?.is_dir() {
                stack.push(entry.path());
            } else {
                if let Some(filename) = entry.path().file_name() {
                    fs::copy(&entry.path(), &dest.join(filename))?;
                }
            }
        }
    }

    Ok(())
}
