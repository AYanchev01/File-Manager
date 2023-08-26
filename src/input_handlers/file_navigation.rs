use crossterm::event::KeyModifiers;
use tui::widgets::ListState;
use crate::fs_utils::FileInfo;
use crate::state::AppState;
use std::process::{Command, Stdio};
use std::env;

const GO_TO_TOP: char = 'g';

pub fn search_files(pattern: &str, files: &[FileInfo], start_index: usize, reverse: bool) -> Option<usize> {
    let regex_match = |index: usize| regex::Regex::new(pattern).ok().map_or(false, |re| re.is_match(&files[index].name));

    if reverse {
        // Start from the file just before the start_index
        if start_index > 0 {
            if let Some(index) = (0..start_index).rev().find(|&i| regex_match(i)) {
                return Some(index);
            }
        }
        // If no match found before the start_index, loop around and search from the end of the list to the start_index.
        (start_index + 1..files.len()).rev().find(|&i| regex_match(i))
    } else {
        // Start from start_index to the end, then loop around from the beginning
        (start_index..files.len()).chain(0..start_index).find(|&i| regex_match(i))
    }
}

pub fn move_in(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], app_state: &mut AppState) {
    if let Some(index) = middle_state.selected() {
        let potential_path = current_dir.join(&files[index].name);
        if potential_path.is_dir() {
            *current_dir = potential_path;
            middle_state.select(Some(0));

        } else if potential_path.is_file() {
            let editor = get_editor();
            
            let result = if cfg!(unix) {
                Command::new(&editor)
                    .arg(potential_path.as_os_str())
                    .stderr(Stdio::null())
                    .status()
            } else if cfg!(windows) {
                Command::new("cmd")
                    .args(["/C", &editor, potential_path.to_str().unwrap()])
                    .stderr(Stdio::null())
                    .status()
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported platform."))
            };
            
            match result {
                Ok(status) if !status.success() => {
                    app_state.prompt_message = Some(format!(" Failed to open file with {}.", &editor));
                }
                Err(_) => {
                    app_state.prompt_message = Some(format!(" Failed to open file with {}.", &editor));
                }
                _ => {}
            }
        }
    }
}

pub fn get_editor() -> String {
    if let Ok(visual) = env::var("VISUAL") {
        if !visual.is_empty() {
            return visual;
        }
    }

    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }

    if cfg!(windows) {
        "notepad.exe".to_string()
    } else {
        "vim".to_string()
    }
}

pub fn move_out(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, left_state: &mut ListState) {
    if let Some(parent) = current_dir.parent() {
        *current_dir = parent.to_path_buf();
        middle_state.select(Some(0));
    } else {
        left_state.select(None);
    }
}

pub fn move_down(middle_state: &mut ListState, max_len: usize,scroll_position: &mut usize,max_scroll: &usize, app_state: &mut AppState) {
    if app_state.last_modifier == Some(KeyModifiers::ALT) {
        if *scroll_position < *max_scroll {
            *scroll_position += 1;
        }
    } else if app_state.last_modifier == Some(KeyModifiers::NONE) {
        adjust_selection(middle_state, max_len, true);
    }
}

pub fn move_up(middle_state: &mut ListState, max_len: usize,scroll_position: &mut usize, app_state: &mut AppState) {
    if app_state.last_modifier == Some(KeyModifiers::ALT) {
        if *scroll_position > 0 {
            *scroll_position -= 1;
        }
    } else if app_state.last_modifier == Some(KeyModifiers::NONE) {
        adjust_selection(middle_state, max_len, false);
    }
}

pub fn move_down_half(middle_state: &mut ListState, files_len: usize, scroll_position: &mut usize,max_scroll: &usize, app_state: &mut AppState) {
    let half_screen = app_state.terminal_height / 2;

    if app_state.last_modifier == Some(KeyModifiers::CONTROL) {
        app_state.last_modifier = Some(KeyModifiers::NONE);

        for _ in 0..half_screen{
            move_down(middle_state, files_len, scroll_position, max_scroll, app_state); 
        }
    } else if app_state.last_modifier == Some(KeyModifiers::ALT) {
        let new_position = *scroll_position + half_screen; 
        if new_position <= *max_scroll {
            *scroll_position = new_position;
        } else {
            *scroll_position = *max_scroll;
        }
    }
}

pub fn move_up_half(middle_state: &mut ListState, files_len: usize, scroll_position: &mut usize, app_state: &mut AppState) {
    let half_screen = app_state.terminal_height / 2;

    if app_state.last_modifier == Some(KeyModifiers::CONTROL) {
        app_state.last_modifier = Some(KeyModifiers::NONE);
 
        for _ in 0..half_screen {
            move_up(middle_state, files_len,scroll_position, app_state); 
        }
    } else if app_state.last_modifier == Some(KeyModifiers::ALT) {
        if *scroll_position >= half_screen{
            *scroll_position -= half_screen;
        } else {
            *scroll_position = 0;
        }
    }
}

pub fn handle_search(app_state: &mut AppState) {
    app_state.search_mode = true;
    app_state.search_pattern = None;
    app_state.prompt_message = Some(String::from(" Searching for: "));
}

pub fn next_search(
    middle_state: &mut ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) {
    if let Some(pattern) = &app_state.search_pattern {
        let start_index = middle_state.selected().unwrap_or(0) + 1; // Start from next index
        app_state.last_search_index = search_files(pattern, files, start_index, false);
        if let Some(index) = app_state.last_search_index {
            middle_state.select(Some(index));
        }
    }
}

pub fn previous_search(
    middle_state: &mut ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) {
    if let Some(pattern) = &app_state.search_pattern {
        let start_index = middle_state.selected().unwrap_or(0); // Start from the current index
        app_state.last_search_index = search_files(pattern, files, start_index, true);
        if let Some(index) = app_state.last_search_index {
            middle_state.select(Some(index));
        }
    }
}

pub fn go_to_top(middle_state: &mut ListState, app_state: &mut AppState,scroll_position: &mut usize) {
    if app_state.last_key_pressed == Some(GO_TO_TOP) {
        if app_state.last_modifier == Some(KeyModifiers::NONE) {
            middle_state.select(Some(0));
        } else {
            *scroll_position = 0;
        }
        app_state.last_key_pressed = None;
    } else {
        app_state.last_key_pressed = Some(GO_TO_TOP);
    }
}

pub fn go_to_bottom(middle_state: &mut ListState, app_state: &mut AppState, files_len: usize, scroll_position: &mut usize, max_scroll: &usize) {
    if app_state.last_modifier == Some(KeyModifiers::SHIFT) {
        if files_len > 0 {
            middle_state.select(Some(files_len - 1));
        }
    } else {
        *scroll_position = *max_scroll;
    }
}

pub fn handle_quit() -> bool {
    true
}

pub fn adjust_selection(state: &mut ListState, max_len: usize, increment: bool) {
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