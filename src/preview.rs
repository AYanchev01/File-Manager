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

pub fn get_file_preview(path: &PathBuf, scroll_position: usize, max_lines: usize) -> Result<(String, usize), Box<dyn std::error::Error>> {
    if is_text_file(path) {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let lines: Vec<_> = contents.lines().collect();
        let start_index = scroll_position.min(lines.len().saturating_sub(max_lines));
        let truncated_contents = lines[start_index..start_index + max_lines.min(lines.len() - start_index)].join("\n");

        let total_lines = lines.len();
        let max_scroll_position = if total_lines > max_lines { total_lines - max_lines } else { 0 };
        Ok((truncated_contents, max_scroll_position))
    } else {
        Ok(("Can't preview file".to_string(),0))
    }
}
