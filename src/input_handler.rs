use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use crate::fs_utils;
use super::fs_utils::*;

const MOVE_DOWN: char = 'j';
const MOVE_UP: char = 'k';
const MOVE_IN: char = 'l';
const MOVE_OUT: char = 'h';
const QUIT: char = 'q';

pub fn handle_input(
    current_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[FileInfo],
    scroll_position: &mut usize,
    max_scroll: &usize,
    selected_file_for_copy: &mut Option<std::path::PathBuf>,
) -> bool {
    match event::read().unwrap() {
        event::Event::Key(KeyEvent { code, .. }) => match code {
            // Navigate down
            KeyCode::Char(MOVE_DOWN) => move_down(middle_state, files.len()),

            // Navigate up
            KeyCode::Char(MOVE_UP) => move_up(middle_state, files.len()),

            // Navigate into directory
            KeyCode::Char(MOVE_IN) => {
                if let Some(index) = middle_state.selected() {
                    let potential_dir = current_dir.join(&files[index].name);
                    if potential_dir.is_dir() {
                        *current_dir = potential_dir;
                        middle_state.select(Some(0));
                    }
                }
            },

            // Navigate to parent directory
            KeyCode::Char(MOVE_OUT) => {
                if let Some(parent) = current_dir.parent() {
                    *current_dir = parent.to_path_buf();
                    middle_state.select(Some(0));
                } else {
                    left_state.select(None);
                }
            },

            // Scroll up
            KeyCode::Up => {
                if *scroll_position > 0 {
                    *scroll_position -= 1;
                }
            },

            // Scroll down
            KeyCode::Down => {
                if *scroll_position < *max_scroll {
                    *scroll_position += 1;
                }
            },

            // Copy file/directory
            KeyCode::Char('y') => {
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
            KeyCode::Char('p') => {
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
            KeyCode::Char('d') => {
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

            // Quit application
            KeyCode::Char(QUIT) | KeyCode::Esc => return true,

            _ => {},
        },
        _ => {},
    }
    false
}