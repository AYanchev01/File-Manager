use std::fs;
use std::path::Path;
use tui::widgets::ListState;

pub fn get_files_and_dirs(dir: &Path) -> Vec<String> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().file_name().unwrap().to_str().unwrap().to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn get_parent_content(dir: &Path) -> Vec<String> {
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

