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
