use crossterm::event::{self};
use tui::widgets::ListState;
use crate::state::AppState;
use super::fs_utils::*;
use crate::input_handlers::modes;

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
    if let Ok(event::Event::Key(key_event)) = event::read() {
        app_state.last_modifier = Some(key_event.modifiers);

        if app_state.delete_mode {
            return modes::handle_delete_mode(key_event.code, current_dir, middle_state, files, app_state);
        } else if app_state.rename_mode {
            return modes::handle_renaming_mode(key_event.code, current_dir, middle_state, files, app_state);
        } else if app_state.search_mode {
            return modes::handle_search_mode(key_event.code, middle_state,files, app_state);
        } else if app_state.is_creating_file || app_state.is_creating_directory {
            return modes::handle_creation_mode(key_event.code, current_dir, app_state);
        } else if app_state.is_changing_permissions {
            return modes::handle_permissions_mode(key_event.code, current_dir, middle_state, files, app_state);
        } else {
            return modes::handle_normal_mode(key_event.code, key_event.modifiers, current_dir, middle_state, left_state, files, scroll_position, max_scroll, selected_file_for_copy, app_state);
        }
    }
    false
}