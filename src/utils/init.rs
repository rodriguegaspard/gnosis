use std::fs::{self, File};
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn ensure_file_exists(filepath: &str) -> io::Result<File> {
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    match File::open(&path) {
        Ok(file) => Ok(file),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            File::create(&path)
        },
        Err(error) => Err(error),
    }
}
