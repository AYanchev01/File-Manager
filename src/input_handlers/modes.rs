use crossterm::event::{KeyCode, KeyModifiers};
use tui::widgets::ListState;
use crate::fs_utils::FileInfo;
use crate::state::AppState;
use super::file_manipulation::*;
use super::file_navigation::*;

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
const YES:                   char = 'y';
const NO:                    char = 'n';
const RENAME:                char = 'r';
const SEARCH:                char = '/';
const NEXT:                  char = 'n';
const PREVIOUS:              char = 'N';
const CREATE_FILE:           char = 'a';
const CREATE_DIR:            char = 'A';
const CHANGE_PERMISSIONS:    char = 'c';

pub fn handle_normal_mode(
    key_code: KeyCode,
    modifiers: KeyModifiers,
    current_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[FileInfo],
    scroll_position: &mut usize,
    max_scroll: &usize,
    app_state: &mut AppState,
) -> bool {
    app_state.prompt_message = None;

    match (key_code, modifiers) {
        (KeyCode::Char(MOVE_IN),_)               => move_in(current_dir, middle_state, files,app_state),
        (KeyCode::Char(MOVE_OUT),_)              => move_out(current_dir, middle_state, left_state),
        (KeyCode::Char(MOVE_UP), _)              => move_up(middle_state,files.len(),scroll_position, app_state),
        (KeyCode::Char(MOVE_DOWN),_)             => move_down(middle_state,files.len(), scroll_position, max_scroll,app_state),
        (KeyCode::Char(MOVE_DOWN_HALF_PAGE), _)  => move_down_half(middle_state, files.len(), scroll_position, max_scroll, app_state),
        (KeyCode::Char(MOVE_UP_HALF_PAGE), _)    => move_up_half(middle_state, files.len(), scroll_position, app_state),
        (KeyCode::Char(CREATE_FILE), _)          => handle_create_file(app_state),
        (KeyCode::Char(CREATE_DIR), _)           => handle_create_directory(app_state),
        (KeyCode::Char(COPY), _)                 => copy_file(current_dir, middle_state, files, app_state),
        (KeyCode::Char(CUT), _)                  => cut_file(current_dir, middle_state, files, app_state),
        (KeyCode::Char(PASTE), _)                => paste_file(current_dir, app_state),
        (KeyCode::Char(DELETE), _)               => handle_delete(middle_state, files, app_state),
        (KeyCode::Char(RENAME), _)               => handle_rename(middle_state, files, app_state),
        (KeyCode::Char(CHANGE_PERMISSIONS), _)   => handle_change_permissions(middle_state, files, app_state),
        (KeyCode::Char(GO_TO_TOP), _)            => go_to_top(middle_state, app_state, scroll_position),
        (KeyCode::Char(GO_TO_BOTTOM), _)         => go_to_bottom(middle_state,app_state, files.len(), scroll_position, max_scroll),
        (KeyCode::Char(SEARCH), _)               => handle_search(app_state),
        (KeyCode::Char(NEXT), _)                 => next_search(middle_state, files, app_state),
        (KeyCode::Char(PREVIOUS), _)             => previous_search(middle_state, files, app_state),
        (KeyCode::Char(QUIT), _)                 => return handle_quit(),
        _                                        => { app_state.last_key_pressed = None; app_state.last_modifier = None; },
    }
    false
}

pub fn handle_creation_mode(
    key_code: KeyCode,
    current_dir: &std::path::PathBuf,
    app_state: &mut AppState,
) -> bool {
    match key_code {
        KeyCode::Char(c) if c != '/' => {
            app_state.creation_buffer.get_or_insert_with(String::new).push(c);
        },
        KeyCode::Backspace => {
            if let Some(buffer) = &mut app_state.creation_buffer {
                buffer.pop();
            }
        },
        KeyCode::Enter => {
            if let Some(new_name) = &app_state.creation_buffer {
                let new_path = current_dir.join(new_name);
                if new_path.exists() {
                    app_state.prompt_message = Some(" Error: File/Directory with this name already exists!".to_string());
                } else {
                    if app_state.is_creating_file {
                        if std::fs::File::create(&new_path).is_err() {
                            app_state.prompt_message = Some(format!(" Failed to create file: {}.", new_name));
                        }
                    } else if app_state.is_creating_directory {
                        if std::fs::create_dir(&new_path).is_err() {
                            app_state.prompt_message = Some(format!(" Failed to create directory: {}.", new_name));
                        }
                    }
                }
                app_state.is_creating_file = false;
                app_state.is_creating_directory = false;
                app_state.creation_buffer = None;
            }
        },
        KeyCode::Esc => {
            app_state.is_creating_file = false;
            app_state.is_creating_directory = false;
            app_state.prompt_message = None;
            app_state.creation_buffer = None;
        },
        _ => {}
    }
    false
}

pub fn handle_search_mode(
    key_code: KeyCode,
    middle_state: &mut ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) -> bool {
    match key_code {
        KeyCode::Char(c) => {
            app_state.search_pattern.get_or_insert_with(String::new).push(c);
        },
        KeyCode::Backspace => {
            if let Some(pattern) = &mut app_state.search_pattern {
                pattern.pop();
            }
        },
        KeyCode::Enter => {
            if let Some(pattern) = &app_state.search_pattern {
                app_state.last_search_index = search_files(pattern, files, 0, false);
            }
            if let Some(index) = app_state.last_search_index {
                middle_state.select(Some(index));
            }
            app_state.search_mode = false;
        },
        KeyCode::Esc => {
            app_state.search_mode = false;
            app_state.search_pattern = None;
        },
        _ => {}
    }

    if let Some(pattern) = &app_state.search_pattern {
        app_state.prompt_message = Some(format!(" Searching for: {}", pattern));
    } else {
        app_state.prompt_message = None;
    }

    false
}

pub fn handle_delete_mode(
    key_code: KeyCode,
    current_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) -> bool {
    match key_code {
        KeyCode::Char(YES) => {
            delete_file(current_dir, middle_state, files, app_state);
            app_state.prompt_message = None;
            app_state.delete_mode = false;
        },
        KeyCode::Char(NO) => {
            app_state.prompt_message = None;
            app_state.delete_mode = false;
        },
        _ => {}
    }
    false
}

pub fn handle_renaming_mode(
    key_code: KeyCode,
    current_dir: &std::path::PathBuf,
    middle_state: &ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) -> bool {
    match key_code {
        KeyCode::Char(c) if c != '/' => {
            app_state.renaming_buffer.get_or_insert_with(String::new).push(c);
        },
        KeyCode::Backspace => {
            if let Some(buffer) = &mut app_state.renaming_buffer {
                buffer.pop();
            }
        },
        KeyCode::Enter => {
            if let Some(index) = middle_state.selected() {
                let file_path = current_dir.join(&files[index].name);
                if let Some(new_name) = &app_state.renaming_buffer {
                    let new_file_path = current_dir.join(new_name);
                    
                    if new_file_path.exists() {
                        app_state.prompt_message = Some(" Error: File with this name already exists!".to_string());
                    } else if let Err(e) = std::fs::rename(&file_path, &new_file_path) {
                        app_state.prompt_message = Some(format!(" Failed to rename {}: {}.", &files[index].name, e));
                    }

                    app_state.rename_mode = false;
                    app_state.renaming_buffer = None;
                }
            }
        },
        KeyCode::Esc => {
            app_state.rename_mode = false;
            app_state.prompt_message = None;
            app_state.renaming_buffer = None;
        },
        _ => {}
    }
    false
}

#[cfg(target_family = "unix")]
pub fn handle_permissions_mode(
    key_code: KeyCode,
    current_dir: &std::path::PathBuf,
    middle_state: &ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) -> bool {
    use std::os::unix::fs::PermissionsExt;

    match key_code {
        KeyCode::Char(c) if c.is_digit(10) && app_state.permissions_buffer.as_ref().unwrap().len() < 3 => {
            app_state.permissions_buffer.get_or_insert_with(String::new).push(c);
        },
        KeyCode::Backspace => {
            if let Some(buffer) = &mut app_state.permissions_buffer {
                buffer.pop();
            }
        },
        KeyCode::Enter => {
            if let Some(index) = middle_state.selected() {
                let file_path = current_dir.join(&files[index].name);
                if let Some(permission_str) = &app_state.permissions_buffer {
                    if let Ok(mode) = u32::from_str_radix(permission_str, 8) {
                        let permissions = std::fs::Permissions::from_mode(mode);
                        if let Err(e) = std::fs::set_permissions(&file_path, permissions) {
                            app_state.prompt_message = Some(format!(" Failed to set permissions for {}: {}.", &files[index].name, e));
                        }
                    } else {
                        app_state.prompt_message = Some(" Invalid permission value!".to_string());
                    }
                    app_state.is_changing_permissions = false;
                    app_state.permissions_buffer = None;
                }
            }
        },
        KeyCode::Esc => {
            app_state.is_changing_permissions = false;
            app_state.prompt_message = None;
            app_state.permissions_buffer = None;
        },
        _ => {}
    }
    false
}

#[cfg(target_family = "windows")]
pub fn handle_permissions_mode(
    _key_code: KeyCode,
    _current_dir: &std::path::PathBuf,
    _middle_state: &ListState,
    _files: &[FileInfo],
    app_state: &mut AppState,
) -> bool {
    app_state.prompt_message = Some(" Changing permissions is not supported on this platform.".to_string());
    false
}