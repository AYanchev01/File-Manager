use std::fs;
use std::path::Path;
use tui::widgets::ListState;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn get_files_and_dirs(dir: &Path) -> Vec<(String, Option<fs::Permissions>, bool)> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let is_dir = path.is_dir();
                let perms = entry.metadata().ok().map(|meta| meta.permissions());
                (name, perms, is_dir)
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn get_parent_content(dir: &Path) -> Vec<(String, Option<fs::Permissions>, bool)> {
    dir.parent()
        .map_or(Vec::new(), |parent| get_files_and_dirs(parent))
}

pub fn move_down(middle_state: &mut ListState, max_len: usize) {
    increment_selection(middle_state, max_len);
}

pub fn move_up(middle_state: &mut ListState, max_len: usize) {
    decrement_selection(middle_state, max_len);
}

fn increment_selection(state: &mut ListState, max_len: usize) {
    if max_len == 0 {
        state.select(None);
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i >= max_len - 1 {
                0
            } else {
                i + 1
            }
        },
        None => 0,
    };
    state.select(Some(i));
}

fn decrement_selection(state: &mut ListState, max_len: usize) {
    if max_len == 0 {
        state.select(None);
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                max_len - 1
            } else {
                i - 1
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
