use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use crate::fs_utils;
use super::fs_utils::*;

const MOVE_DOWN: char = 'j';
const MOVE_UP: char = 'k';
const MOVE_IN: char = 'l';
const MOVE_OUT: char = 'h';
const QUIT: char = 'q';
const GO_TO_TOP: char = 'g';
const GO_TO_BOTTOM: char = 'G';
const END_OF_FILE: char = '$';
const BEGINNING_OF_FILE: char = '^';
const COPY: char = 'y';
const PASTE: char = 'p';
const DELETE: char = 'd';
const MOVE_UP_HALF_PAGE: char = 'u';
const MOVE_DOWN_HALF_PAGE: char = 'd';

pub fn handle_input(
    current_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[FileInfo],
    scroll_position: &mut usize,
    max_scroll: &usize,
    selected_file_for_copy: &mut Option<std::path::PathBuf>,
) -> bool {
    // Calculate the half screen size
    let terminal_size = crossterm::terminal::size().unwrap();
    let half_screen = (terminal_size.1 as usize - 4) * 95 / 200;

    static mut LAST_KEY_PRESSED: Option<char> = None;

    match event::read().unwrap() {
        event::Event::Key(KeyEvent { code, modifiers, .. }) => match (code,modifiers) {
            // Navigate into directory
            (KeyCode::Char(MOVE_IN),_) => {
                if let Some(index) = middle_state.selected() {
                    let potential_dir = current_dir.join(&files[index].name);
                    if potential_dir.is_dir() {
                        *current_dir = potential_dir;
                        middle_state.select(Some(0));
                    }
                }
            },

            // Navigate to parent directory
            (KeyCode::Char(MOVE_OUT),_) => {
                if let Some(parent) = current_dir.parent() {
                    *current_dir = parent.to_path_buf();
                    middle_state.select(Some(0));
                } else {
                    left_state.select(None);
                }
            },

            // Scroll up
            (KeyCode::Char(MOVE_UP), crossterm::event::KeyModifiers::ALT) => {
                if *scroll_position > 0 {
                    *scroll_position -= 1;
                }
            },

            // Scroll down
            (KeyCode::Char(MOVE_DOWN), crossterm::event::KeyModifiers::ALT) => {
                if *scroll_position < *max_scroll {
                    *scroll_position += 1;
                }
            },

            // Scroll down by half a screen
            (KeyCode::Char(MOVE_DOWN_HALF_PAGE), crossterm::event::KeyModifiers::CONTROL) => {
                for _ in 0..half_screen{
                    move_down(middle_state, files.len());
                }
            },

            // Scroll up by half a screen
            (KeyCode::Char(MOVE_UP_HALF_PAGE), crossterm::event::KeyModifiers::CONTROL) => {
                for _ in 0..half_screen{
                    move_up(middle_state, files.len());
                }
            },

            // Navigate down
            (KeyCode::Char(MOVE_DOWN),_) => move_down(middle_state, files.len()),

            // Navigate up
            (KeyCode::Char(MOVE_UP),_) => move_up(middle_state, files.len()),

            // Scroll up by half a screen
            (KeyCode::Char(MOVE_UP_HALF_PAGE), crossterm::event::KeyModifiers::ALT) => {
                if *scroll_position >= half_screen {
                    *scroll_position -= half_screen;
                } else {
                    *scroll_position = 0;
                }
            },

            // Scroll down by half a screen
            (KeyCode::Char(MOVE_DOWN_HALF_PAGE), crossterm::event::KeyModifiers::ALT) => {
                let new_position = *scroll_position + half_screen;
                if new_position <= *max_scroll {
                    *scroll_position = new_position;
                } else {
                    *scroll_position = *max_scroll;
                }
            },

            // Copy file/directory
            (KeyCode::Char(COPY), _) => {
                if let Some(index) = middle_state.selected() {
                    if index < files.len() {
                        let potential_file = current_dir.join(&files[index].name);
                        if potential_file.exists() {
                            *selected_file_for_copy = Some(potential_file);
                        }
                    }
                }
            }

            // Paste file/directory
            (KeyCode::Char(PASTE), _) => {
                if let Some(ref src) = *selected_file_for_copy {
                    let dest = make_unique_path(current_dir.join(src.file_name().unwrap_or_default()));
                    match fs_utils::copy(src, &dest) {
                        Ok(_) => {},
                        Err(e) => {
                            // Just print error message for now
                            println!("Error while copying: {}", e);
                        }
                    }
                    // *selected_file_for_copy = None;
                }
            },

            // Delete file/directory
            (KeyCode::Char(DELETE), _) => {
                if let Some(index) = middle_state.selected() {
                    if index < files.len() {
                        let potential_file = current_dir.join(&files[index].name);
                        match fs_utils::delete(&potential_file) {
                            Ok(_) => {},
                            Err(e) => {
                                // Just print error message for now
                                println!("Error while copying: {}", e);
                            }
                        }
                    }
                }
            },

            // Scroll to the first line of text preview
            (KeyCode::Char(BEGINNING_OF_FILE), crossterm::event::KeyModifiers::ALT) => {
                *scroll_position = 0;
            },

            // Scroll to the last line of text preview
            (KeyCode::Char(END_OF_FILE), crossterm::event::KeyModifiers::ALT) => {
                *scroll_position = *max_scroll;
            },

            // Navigate to the first file with "gg"
            (KeyCode::Char(GO_TO_TOP), _) => {
                if unsafe { LAST_KEY_PRESSED } == Some(GO_TO_TOP) {
                    middle_state.select(Some(0));
                    unsafe {
                        LAST_KEY_PRESSED = None;
                    }
                } else {
                    unsafe {
                        LAST_KEY_PRESSED = Some(GO_TO_TOP);
                    }
                }
            }

            // Navigate to the last file with "G"
            (KeyCode::Char(GO_TO_BOTTOM), _) => {
                if files.len() > 0 {
                    middle_state.select(Some(files.len() - 1));
                }
            }

            // Quit application
            (KeyCode::Char(QUIT), _) | (KeyCode::Esc, _) => return true,

                _ => {
                    unsafe {
                        LAST_KEY_PRESSED = None;
                    }
                },
            }
        _ => {},
    }
    false
}