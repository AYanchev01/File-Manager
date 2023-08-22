use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tui::widgets::ListState;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct FileInfo {
    pub name: String,
    pub perms: Option<fs::Permissions>,
    pub is_dir: bool,
    pub is_exec: bool,
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

pub fn move_down(middle_state: &mut ListState, max_len: usize) {
    adjust_selection(middle_state, max_len, true);
}

pub fn move_up(middle_state: &mut ListState, max_len: usize) {
    adjust_selection(middle_state, max_len, false);
}

fn adjust_selection(state: &mut ListState, max_len: usize, increment: bool) {
    if max_len == 0 {
        state.select(None);
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if increment {
                if i >= max_len - 1 { i } else { i + 1 }
            } else {
                if i == 0 { i } else { i - 1 }
            }
        },
        None => 0,
    };
    state.select(Some(i));
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