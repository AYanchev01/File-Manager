use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use super::fs_utils::*;

pub fn handle_input(
    current_dir: &mut std::path::PathBuf,
    selected_dir: &mut std::path::PathBuf,
    middle_state: &mut ListState,
    left_state: &mut ListState,
    files: &[String]
) -> bool {
    match event::read().unwrap() {
        event::Event::Key(KeyEvent { code, .. }) => {
            match code {
                KeyCode::Char('j') => {
                    move_down(middle_state, files.len());
                }
                KeyCode::Char('k') => {
                    move_up(middle_state, files.len());
                }
                KeyCode::Char('l') => {
                    if !selected_dir.as_os_str().is_empty() {
                        *current_dir = selected_dir.clone();
                        middle_state.select(Some(0));
                    }
                }
                KeyCode::Char('h') => {
                    if let Some(parent) = current_dir.parent() {
                        *current_dir = parent.to_path_buf();
                    } else {
                        left_state.select(None);
                    }
                    middle_state.select(Some(0));
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    return true;
                }
                _ => {}
            }
        }
        _ => {}
    }
    false
}
