use tui::widgets::ListState;
use crate::fs_utils::{FileInfo, make_unique_path};
use crate::{fs_utils, AppState};

pub fn handle_create_file(app_state: &mut AppState) {
    app_state.is_creating_file = true;
    app_state.prompt_message = Some(" Create new file: ".to_string());
    app_state.creation_buffer = Some(String::new());
}

pub fn handle_create_directory(app_state: &mut AppState) {
    app_state.is_creating_directory = true;
    app_state.prompt_message = Some(" Create new directory: ".to_string());
    app_state.creation_buffer = Some(String::new());
}

#[cfg(target_family = "unix")]
pub fn handle_change_permissions(
    middle_state: &ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) {
    if let Some(index) = middle_state.selected() {
        let file_name = &files[index].name;
        app_state.is_changing_permissions = true;
        app_state.prompt_message = Some(format!(" Change permissions of \"{}\": ", file_name));
        app_state.permissions_buffer = Some(String::new());
    }
}

#[cfg(target_family = "windows")]
pub fn handle_change_permissions(
    _middle_state: &ListState,
    _files: &[FileInfo],
    app_state: &mut AppState,
) {
    app_state.prompt_message = Some(" Changing permissions is not supported on this platform.".to_string());
}

pub fn copy_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
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

pub fn cut_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
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

pub fn paste_file(current_dir: &mut std::path::PathBuf, selected_file_for_copy: &mut Option<std::path::PathBuf>, app_state: &mut AppState) {
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
                    app_state.prompt_message = Some(format!(" Error while moving: {}", e));
                }
            }
        } else {
            match fs_utils::copy(src, &dest) {
                Ok(_) => {},
                Err(e) => {
                    app_state.prompt_message = Some(format!(" Error while copying: {}", e));
                }
            }
        }
        *selected_file_for_copy = None;
        app_state.was_cut = false;
    }
}

pub fn handle_delete(middle_state: &mut ListState, files: &[FileInfo], app_state: &mut AppState) {
    if app_state.prompt_message.is_none() {
        if let Some(index) = middle_state.selected() {
            if index < files.len() {
                let file_name = &files[index].name;
                app_state.prompt_message = Some(format!(" Are you sure you want to delete {}? (y/n)", file_name));
                app_state.delete_mode = true;
            }
        }
    }
}

pub fn delete_file(current_dir: &mut std::path::PathBuf, middle_state: &mut ListState, files: &[FileInfo], app_state: &mut AppState) {
    if let Some(index) = middle_state.selected() {
        if index < files.len() {
            let potential_file = current_dir.join(&files[index].name);
            match fs_utils::delete(&potential_file) {
                Ok(_) => {},
                Err(e) => {
                    app_state.prompt_message = Some(format!(" Error while deleting: {}", e));
                }
            }
        }
    }
}

pub fn handle_rename(
    middle_state: &ListState,
    files: &[FileInfo],
    app_state: &mut AppState,
) {
    if let Some(index) = middle_state.selected() {
        let file_name = &files[index].name;
        app_state.rename_mode = true;
        app_state.prompt_message = Some(format!(" Rename \"{}\" to: ", file_name));
        app_state.renaming_buffer = Some(String::new());
    }
}