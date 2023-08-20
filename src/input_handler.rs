use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use super::fs_utils::*;

const MOVE_DOWN: char = 'j';
const MOVE_UP: char = 'k';
const MOVE_IN: char = 'l';
const MOVE_OUT: char = 'h';
const QUIT: char = 'q';

pub fn handle_input(
    current_dir: &mut std::path::PathBuf,
    selected_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[FileInfo],
    scroll_position: &mut usize,
    max_scroll: &usize,
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

            // Quit application
            KeyCode::Char(QUIT) | KeyCode::Esc => return true,

            _ => {},
        },
        _ => {},
    }
    false
}