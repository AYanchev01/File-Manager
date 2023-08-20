use crossterm::event::{self, KeyCode, KeyEvent};
use tui::widgets::ListState;
use super::fs_utils::*;

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
        event::Event::Key(KeyEvent { code, .. }) => {
            match code {
                KeyCode::Char('j') => {
                    move_down(middle_state, files.len());
                }
                KeyCode::Char('k') => {
                    move_up(middle_state, files.len());
                }
                KeyCode::Char('l') => {
                    if let Some(index) = middle_state.selected() {
                        if index < files.len() {
                            let potential_dir = current_dir.join(&files[index].name);
                            if potential_dir.is_dir() {
                                *current_dir = potential_dir;
                                middle_state.select(Some(0));
                            }
                        }
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
                KeyCode::Up => {
                    if *scroll_position > 0 {
                        *scroll_position -= 1;
                    }
                }
                KeyCode::Down => {
                    if *scroll_position < *max_scroll {
                        *scroll_position += 1;
                    }
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