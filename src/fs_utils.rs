use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use tui::widgets::ListState;

use crate::preview::get_file_preview;

pub struct FileInfo {
    pub name: String,
    pub perms: Option<fs::Permissions>,
    pub is_dir: bool,
    pub is_exec: bool,
}

pub fn update_selected_dir(files: &[FileInfo], current_dir: &std::path::PathBuf, selected_dir: &mut std::path::PathBuf, middle_state: &ListState, scroll_position: &mut usize) {
    if let Some(selected) = middle_state.selected() {
        if selected < files.len() {
            let path = current_dir.join(&files[selected].name);
            if !files[selected].is_dir && selected_dir != &path {
                *scroll_position = 0;
                *selected_dir = path;
            } else if files[selected].is_dir {
                *selected_dir = path;
            }
        }
    }
}

pub fn fetch_children(selected_dir: &std::path::PathBuf, scroll_position: usize, right_pane_height: usize) -> (Vec<FileInfo>, usize) {
    if selected_dir.as_os_str().is_empty() {
        return (vec![create_file_info("Select a directory or file".to_string())], 0);
    } else if selected_dir.is_file() {
        match get_file_preview(&selected_dir, scroll_position, right_pane_height) {
            Ok((preview_text, max_scroll_position)) => {
                (vec![create_file_info(preview_text)], max_scroll_position)
            },
            Err(_) => {
                (vec![create_file_info("Failed to load file preview".to_string())], 0)
            }
        }
    } else {
        let contents = get_files_and_dirs(&selected_dir);
        if contents.is_empty() {
            return (vec![create_file_info("empty".to_string())], 0);
        } else {
            return (contents, 0);
        }
    }
}

pub fn create_file_info(name: String) -> FileInfo {
    FileInfo {
        name,
        perms: None,
        is_dir: false,
        is_exec: false,
    }
}
pub fn get_files_and_dirs(dir: &Path) -> Vec<FileInfo> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let is_dir = path.is_dir();
                let perms = entry.metadata().ok().map(|meta| meta.permissions());
                let is_exec = perms.as_ref().map_or(false, |p| is_executable(p));
                FileInfo {
                    name,
                    perms,
                    is_dir,
                    is_exec
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn get_parent_content(dir: &Path) -> Vec<FileInfo> {
    dir.parent()
        .map_or(Vec::new(), |parent| get_files_and_dirs(parent))
}

pub fn get_permissions(metadata: &fs::Permissions) -> String {
    #[cfg(unix)]
    {
        let perms_unix = metadata.mode();
        format!(
            "{}{}{}{}{}{}{}{}{}",
            if perms_unix & 0o400 != 0 { "r" } else { "-" },
            if perms_unix & 0o200 != 0 { "w" } else { "-" },
            if perms_unix & 0o100 != 0 { "x" } else { "-" },
            if perms_unix & 0o040 != 0 { "r" } else { "-" },
            if perms_unix & 0o020 != 0 { "w" } else { "-" },
            if perms_unix & 0o010 != 0 { "x" } else { "-" },
            if perms_unix & 0o004 != 0 { "r" } else { "-" },
            if perms_unix & 0o002 != 0 { "w" } else { "-" },
            if perms_unix & 0o001 != 0 { "x" } else { "-" }
        )
    }
    #[cfg(windows)]
    {
        if metadata.readonly() {
            "r--r--r--".to_string()
        } else {
            "rw-rw-rw-".to_string()
        }
    }
}

#[cfg(unix)]
pub fn is_executable(metadata: &fs::Permissions) -> bool {
    let perms_unix = metadata.mode();
    (perms_unix & 0o100 != 0) || (perms_unix & 0o010 != 0) || (perms_unix & 0o001 != 0)
}

#[cfg(windows)]
pub fn is_executable(_metadata: &fs::Permissions) -> bool {
    // Windows executability is not determined solely by file permissions.
    false
}

pub fn make_unique_path(mut path: PathBuf) -> PathBuf {
    let original_path = path.clone();
    let mut counter = 1;

    while path.exists() {
        if let Some(extension) = original_path.extension() {
            let new_stem = format!("{}_{}", original_path.file_stem().unwrap().to_string_lossy(), counter);
            path.set_file_name(new_stem);
            path.set_extension(extension);
        } else {
            let new_name = format!("{}_{}", original_path.file_name().unwrap().to_string_lossy(), counter);
            path.set_file_name(&new_name);
        }
        counter += 1;
    }

    path
}

pub fn copy(src: &Path, dest: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_to(src, dest)
    } else {
        paste_file(src, dest)
    }
}

pub fn delete(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        delete_dir(path)
    } else {
        delete_file(path)
    }
}

pub fn paste_file(src: &Path, dest: &Path) -> std::io::Result<()> {
    let mut src_file = fs::File::open(src)?;
    let mut contents = Vec::new();
    src_file.read_to_end(&mut contents)?;

    let mut dest_file = fs::File::create(dest)?;
    dest_file.write_all(&contents)?;

    Ok(())
}

pub fn copy_dir_to(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }
    
    for entry_result in fs::read_dir(src)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_to(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(&entry.path(), &dst.join(entry.file_name()))?;
        }
    }
    
    Ok(())
}

pub fn delete_file(path: &Path) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn delete_dir(path: &Path) -> std::io::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

// Try unix rename; only works if src & dest are on the same fs; copy+delete otherwise 
pub fn move_file(src: &Path, dest: &Path) -> std::io::Result<()> {
    if let Err(_) = fs::rename(src, dest) {
        copy(src, dest)?;
        delete(src)?;
    }

    Ok(())
}
