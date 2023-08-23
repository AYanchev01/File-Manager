use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use tui::widgets::ListState;
use crate::{fs_utils, AppState};
use super::fs_utils::*;

const MOVE_DOWN:             char = 'j';
const MOVE_UP:               char = 'k';
const MOVE_IN:               char = 'l';
const MOVE_OUT:              char = 'h';
const QUIT:                  char = 'q';
const GO_TO_TOP:             char = 'g';
const GO_TO_BOTTOM:          char = 'G';
const COPY:                  char = 'y';
const PASTE:                 char = 'p';
const DELETE:                char = 'D';
const CUT:                   char = 'x';
const MOVE_UP_HALF_PAGE:     char = 'u';
const MOVE_DOWN_HALF_PAGE:   char = 'd';

pub fn handle_input(
    current_dir:             &mut std::path::PathBuf,
    middle_state:            &mut ListState,
    left_state:              &mut ListState,
    files:                   &[FileInfo],
    scroll_position:         &mut usize,
    max_scroll:              &usize,
    selected_file_for_copy:  &mut Option<std::path::PathBuf>,
    app_state:               &mut AppState,
) -> bool {

    match event::read().unwrap() {
        event::Event::Key(KeyEvent { code, modifiers, .. }) => {
            app_state.last_modifier = Some(modifiers);

            match (code,modifiers) {
                (KeyCode::Char(MOVE_IN),_)               => move_in(current_dir, middle_state, files),
                (KeyCode::Char(MOVE_OUT),_)              => move_out(current_dir, middle_state, left_state),
                (KeyCode::Char(MOVE_UP), _)              => move_up(middle_state,files.len(),scroll_position, app_state),
                (KeyCode::Char(MOVE_DOWN),_)             => move_down(middle_state,files.len(), scroll_position, max_scroll,app_state),
                (KeyCode::Char(MOVE_DOWN_HALF_PAGE), _)  => move_down_half(middle_state, files.len(), scroll_position, max_scroll, app_state),
                (KeyCode::Char(MOVE_UP_HALF_PAGE), _)    => move_up_half(middle_state, files.len(), scroll_position, app_state),
                (KeyCode::Char(COPY), _)                 => copy_file(current_dir, middle_state, files, selected_file_for_copy, app_state),
                (KeyCode::Char(CUT), _)                  => cut_file(current_dir, middle_state, files, selected_file_for_copy, app_state),
                (KeyCode::Char(PASTE), _)                => paste_file(current_dir, selected_file_for_copy, app_state),
                (KeyCode::Char(DELETE), _)               => delete_file(current_dir, middle_state, files),
                (KeyCode::Char(GO_TO_TOP), _)            => go_to_top(middle_state, app_state, scroll_position),
                (KeyCode::Char(GO_TO_BOTTOM), _)         => go_to_bottom(middle_state,app_state, files.len(), scroll_position, max_scroll),
                (KeyCode::Char(QUIT), _)                 => return handle_quit(),
                _                                        => { app_state.last_key_pressed = None; app_state.last_modifier = None; },
            }
        }
        _ => {},
    }
    false
}

fn move_in(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo]) {
    if let Some(index) = middle_state.selected() {
        let potential_dir = current_dir.join(&files[index].name);
        if potential_dir.is_dir() {
            *current_dir = potential_dir;
            middle_state.select(Some(0));
        }
    }
}

fn move_out(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, left_state: &mut ListState) {
    if let Some(parent) = current_dir.parent() {
        *current_dir = parent.to_path_buf();
        middle_state.select(Some(0));
    } else {
        left_state.select(None);
    }
}

fn move_down(middle_state: &mut ListState, max_len: usize,scroll_position: &mut usize,max_scroll: &usize, app_state: &mut AppState) {
    if app_state.last_modifier == Some(KeyModifiers::ALT) {
        if *scroll_position < *max_scroll {
            *scroll_position += 1;
        }
    } else if app_state.last_modifier == Some(KeyModifiers::NONE) {
        adjust_selection(middle_state, max_len, true);
    }
}

fn move_up(middle_state: &mut ListState, max_len: usize,scroll_position: &mut usize, app_state: &mut AppState) {
    if app_state.last_modifier == Some(KeyModifiers::ALT) {
        if *scroll_position > 0 {
            *scroll_position -= 1;
        }
    } else if app_state.last_modifier == Some(KeyModifiers::NONE) {
        adjust_selection(middle_state, max_len, false);
    }
}

fn move_down_half(middle_state: &mut ListState, files_len: usize, scroll_position: &mut usize,max_scroll: &usize, app_state: &mut AppState) {
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

fn move_up_half(middle_state: &mut ListState, files_len: usize, scroll_position: &mut usize, app_state: &mut AppState) {
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

fn go_to_top(middle_state: &mut ListState, app_state: &mut AppState,scroll_position: &mut usize) {
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

fn go_to_bottom(middle_state: &mut ListState, app_state: &mut AppState, files_len: usize, scroll_position: &mut usize, max_scroll: &usize) {
    if app_state.last_modifier == Some(KeyModifiers::SHIFT) {
        if files_len > 0 {
            middle_state.select(Some(files_len - 1));
        }
    } else {
        *scroll_position = *max_scroll;
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