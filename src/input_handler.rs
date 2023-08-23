use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use crate::{fs_utils, AppState};
use super::fs_utils::*;

const MOVE_DOWN:            char = 'j';
const MOVE_UP:              char = 'k';
const MOVE_IN:              char = 'l';
const MOVE_OUT:             char = 'h';
const QUIT:                 char = 'q';
const GO_TO_TOP:            char = 'g';
const GO_TO_BOTTOM:         char = 'G';
const END_OF_FILE:          char = '$';
const BEGINNING_OF_FILE:    char = '^';
const COPY:                 char = 'y';
const PASTE:                char = 'p';
const DELETE:               char = 'd';
const CUT:                  char = 'x';
const MOVE_UP_HALF_PAGE:    char = 'u';
const MOVE_DOWN_HALF_PAGE:  char = 'd';

pub fn handle_input(
    current_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[FileInfo],
    scroll_position: &mut usize,
    max_scroll: &usize,
    selected_file_for_copy: &mut Option<std::path::PathBuf>,
    app_state: &mut AppState,
) -> bool {

    match event::read().unwrap() {
        event::Event::Key(KeyEvent { code, modifiers, .. }) => match (code,modifiers) {
            (KeyCode::Char(MOVE_IN),_) => navigate_in(current_dir, middle_state, files),
            (KeyCode::Char(MOVE_OUT),_) => navigate_out(current_dir, middle_state, left_state),
            (KeyCode::Char(MOVE_UP), crossterm::event::KeyModifiers::ALT) => scroll_up_alt(scroll_position),
            (KeyCode::Char(MOVE_DOWN), crossterm::event::KeyModifiers::ALT) => scroll_down_alt(scroll_position, max_scroll),
            (KeyCode::Char(MOVE_DOWN_HALF_PAGE), crossterm::event::KeyModifiers::CONTROL) => scroll_down_half_ctrl(middle_state, files.len(), app_state.terminal_height / 2),
            (KeyCode::Char(MOVE_UP_HALF_PAGE), crossterm::event::KeyModifiers::CONTROL) => scroll_up_half_ctrl(middle_state, files.len(), app_state.terminal_height / 2),
            (KeyCode::Char(MOVE_DOWN),_) => move_down(middle_state, files.len()),
            (KeyCode::Char(MOVE_UP),_) => move_up(middle_state, files.len()),
            (KeyCode::Char(MOVE_DOWN_HALF_PAGE), crossterm::event::KeyModifiers::ALT) => scroll_down_half_alt(scroll_position, app_state.terminal_height / 2, max_scroll),
            (KeyCode::Char(MOVE_UP_HALF_PAGE), crossterm::event::KeyModifiers::ALT) => scroll_up_half_alt(scroll_position, app_state.terminal_height / 2),
            (KeyCode::Char(COPY), _) => copy_file(current_dir, middle_state, files, selected_file_for_copy, app_state),
            (KeyCode::Char(CUT), _) => cut_file(current_dir, middle_state, files, selected_file_for_copy, app_state),
            (KeyCode::Char(PASTE), _) => paste_file(current_dir, selected_file_for_copy, app_state),
            (KeyCode::Char(DELETE), _) => delete_file(current_dir, middle_state, files),
            (KeyCode::Char(BEGINNING_OF_FILE), crossterm::event::KeyModifiers::ALT) => scroll_to_top_alt(scroll_position),
            (KeyCode::Char(END_OF_FILE), crossterm::event::KeyModifiers::ALT) => scroll_to_end_alt(scroll_position, max_scroll),
            (KeyCode::Char(GO_TO_TOP), _) => go_to_top(middle_state, app_state),
            (KeyCode::Char(GO_TO_BOTTOM), _) => go_to_bottom(middle_state, files.len()),
            (KeyCode::Char(QUIT), _) | (KeyCode::Esc, _) => return handle_quit(),
            _ => {
                app_state.last_key_pressed = None;
            },
        }
        _ => {},
    }
    false
}

fn navigate_in(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo]) {
    if let Some(index) = middle_state.selected() {
        let potential_dir = current_dir.join(&files[index].name);
        if potential_dir.is_dir() {
            *current_dir = potential_dir;
            middle_state.select(Some(0));
        }
    }
}

fn navigate_out(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, left_state: &mut ListState) {
    if let Some(parent) = current_dir.parent() {
        *current_dir = parent.to_path_buf();
        middle_state.select(Some(0));
    } else {
        left_state.select(None);
    }
}

fn scroll_up_alt(scroll_position: &mut usize) {
    if *scroll_position > 0 {
        *scroll_position -= 1;
    }
}

fn scroll_down_alt(scroll_position: &mut usize, max_scroll: &usize) {
    if *scroll_position < *max_scroll {
        *scroll_position += 1;
    }
}

fn scroll_down_half_ctrl(middle_state: &mut ListState, files_len: usize, half_screen: usize) {
    for _ in 0..half_screen{
        move_down(middle_state, files_len); 
    }
}

fn scroll_up_half_ctrl(middle_state: &mut ListState, files_len: usize, half_screen: usize) {
    for _ in 0..half_screen{
        move_up(middle_state, files_len); 
    }
}

fn move_down(middle_state: &mut ListState, max_len: usize) {
    adjust_selection(middle_state, max_len, true);
}

fn move_up(middle_state: &mut ListState, max_len: usize) {
    adjust_selection(middle_state, max_len, false);
}

fn scroll_down_half_alt(scroll_position: &mut usize ,half_screen: usize, max_scroll: &usize) {
    let new_position = *scroll_position + half_screen;
    if new_position <= *max_scroll {
        *scroll_position = new_position;
    } else {
        *scroll_position = *max_scroll;
    }
}

fn scroll_up_half_alt(scroll_position: &mut usize, half_screen: usize) {
    if *scroll_position >= half_screen {
        *scroll_position -= half_screen;
    } else {
        *scroll_position = 0;
    }
}

fn copy_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
    if let Some(index) = middle_state.selected() {
        if index < files.len() {
            let potential_file = current_dir.join(&files[index].name);
            if potential_file.exists() {
                *selected_file_for_copy = Some(potential_file);
                app_state.was_cut = false;
            }
        }
    }
}

fn cut_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
    if let Some(index) = middle_state.selected() {
        if index < files.len() {
            let potential_file = current_dir.join(&files[index].name);
            if potential_file.exists() {
                *selected_file_for_copy = Some(potential_file);
                app_state.was_cut = true;
            }
        }
    }
}

fn paste_file(current_dir: &mut std::path::PathBuf, selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
    if let Some(ref src) = *selected_file_for_copy {
        let original_dest = current_dir.join(src.file_name().unwrap_or_default());
        
        // If the file was cut use the original dest, otherwise make it unique for copy
        let dest = if app_state.was_cut {
            original_dest
        } else {
            make_unique_path(original_dest)
        };

        if app_state.was_cut {
            match fs_utils::move_file(src, &dest) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error while moving: {}", e);
                }
            }
        } else {
            match fs_utils::copy(src, &dest) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error while copying: {}", e);
                }
            }
        }
        *selected_file_for_copy = None;
        app_state.was_cut = false;
    }
}

fn delete_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo]) {
    if let Some(index) = middle_state.selected() {
        if index < files.len() {
            let potential_file = current_dir.join(&files[index].name);
            match fs_utils::delete(&potential_file) {
                Ok(_) => {},
                Err(e) => {
                    // Just print error message for now
                    println!("Error while deleting: {}", e);
                }
            }
        }
    }
}

fn scroll_to_top_alt(scroll_position: &mut usize) {
    *scroll_position = 0;
}

fn scroll_to_end_alt(scroll_position: &mut usize, max_scroll: &usize) {
    *scroll_position = *max_scroll;
}

fn go_to_top(middle_state: &mut ListState, app_state: &mut AppState) {
    if app_state.last_key_pressed == Some(GO_TO_TOP) {
        middle_state.select(Some(0));
        app_state.last_key_pressed = None;
    } else {
        app_state.last_key_pressed = Some(GO_TO_TOP);
    }
}

fn go_to_bottom(middle_state: &mut ListState, files_len: usize) {
    if files_len > 0 {
        middle_state.select(Some(files_len - 1));
    }
}

fn handle_quit() -> bool {
    true
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