use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file(filepath: &str) -> io::Result<String> {
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        fs::File::create(path)?;
        Ok("".to_string())
    } else {
        let mut content = String::new();
        fs::File::open(path)?.read_to_string(&mut content)?;
        Ok(content)
    }
}

pub fn read_directory(dirpath: &str) -> io::Result<Vec<(String, String)>> {
    let path = Path::new(dirpath);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    let mut files_content = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            let title = entry.file_name().to_string_lossy().into_owned();
            let mut file_content = String::new();
            let mut file = fs::File::open(entry.path())?;
            file.read_to_string(&mut file_content)?;
            files_content.push((title, file_content));
        }
    }
    Ok(files_content)
}
