use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;

fn is_text_file(path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(path) {
        let mut buffer = vec![0; 512]; // Read the first 512 bytes
        if let Ok(n) = file.read(&mut buffer) {
            buffer.truncate(n);
            return !buffer.contains(&0); // If it contains a null byte, probably binary
        }
    }
    false
}

pub fn get_file_preview(path: &PathBuf, max_lines: usize) -> Result<String, Box<dyn std::error::Error>> {
    if is_text_file(path) {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let truncated_contents = contents.lines()
            .take(max_lines)
            .collect::<Vec<_>>()
            .join("\n");

        Ok(truncated_contents)
    } else {
        Ok("Can't preview file".to_string())
    }
}
