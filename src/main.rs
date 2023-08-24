extern crate tui;
extern crate crossterm;

use crossterm::terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use tui::Terminal;
use tui::widgets::{ListState, Paragraph};
use std::env;

mod ui;
mod fs_utils;
mod input_handler;
mod preview;

use ui::{render_pane, PaneType};
use fs_utils::*;
use input_handler::*;
use preview::*;

pub struct AppState {
    last_key_pressed: Option<char>,
    last_modifier: Option<crossterm::event::KeyModifiers>,
    was_cut: bool,
    terminal_height: usize,
    is_delete_prompt: bool,
    prompt_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        // Calculate the half screen size
        let terminal_size = crossterm::terminal::size().unwrap();

        AppState {
            last_key_pressed: None,
            last_modifier: None,
            was_cut: false,
            terminal_height : (terminal_size.1 as usize - 4) * 90 / 100,
            is_delete_prompt: false,
            prompt_message: None,
        }
    }
}

fn main() {
    // Initialize crossterm
    terminal::enable_raw_mode().unwrap();
    
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut app_state = AppState::new();

    // Initialize directories and states
    let mut current_dir = env::current_dir().unwrap();
    let mut selected_dir = current_dir.clone();
    let mut scroll_position = 0;
    let mut selected_file_for_copy: Option<std::path::PathBuf> = None;

    let mut left_state = ListState::default();
    let mut middle_state = ListState::default();
    let mut right_state = ListState::default();

    middle_state.select(Some(0));

    loop {
        let parents = get_parent_content(&current_dir);
        let files = get_files_and_dirs(&current_dir);
        update_selected_dir(&files, &current_dir, &mut selected_dir, &middle_state, &mut scroll_position);

        let (children, max_scroll) = fetch_children(&selected_dir, scroll_position, app_state.terminal_height);

        // Render UI
        terminal.draw(|f| {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(95), // height of top section
                        Constraint::Percentage(5), // height of bottom text pane 
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(30),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(vertical_chunks[0]);

            render_pane(f, horizontal_chunks[0], &parents, &mut left_state, PaneType::Left);
            render_pane(f, horizontal_chunks[1], &files, &mut middle_state, PaneType::Middle);
            render_pane(f, horizontal_chunks[2], &children, &mut right_state, PaneType::Right);

            // Render the small horizontal pane for displaying text
            let text_to_display = app_state.prompt_message.as_deref().unwrap_or_default();
            let text_paragraph = Paragraph::new(text_to_display);
            f.render_widget(text_paragraph, vertical_chunks[1]);

        }).unwrap();

        // Handle input
        if handle_input(&mut current_dir, &mut middle_state, &mut left_state, &files, &mut scroll_position, &max_scroll, &mut selected_file_for_copy, &mut app_state) {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
}

fn update_selected_dir(files: &[FileInfo], current_dir: &std::path::PathBuf, selected_dir: &mut std::path::PathBuf, middle_state: &ListState, scroll_position: &mut usize) {
    if let Some(selected) = middle_state.selected() {
        if selected < files.len() {
            let path = current_dir.join(&files[selected].name);
            if !files[selected].is_dir && selected_dir != &path {
                *scroll_position = 0;
                *selected_dir = path;
            } else if files[selected].is_dir {
                *selected_dir = path;
            }
        }
    }
}

fn fetch_children(selected_dir: &std::path::PathBuf, scroll_position: usize, right_pane_height: usize) -> (Vec<FileInfo>, usize) {
    if selected_dir.as_os_str().is_empty() {
        return (vec![create_file_info("Select a directory or file".to_string())], 0);
    } else if selected_dir.is_file() {
        match get_file_preview(&selected_dir, scroll_position, right_pane_height) {
            Ok((preview_text, max_scroll_position)) => {
                (vec![create_file_info(preview_text)], max_scroll_position)
            },
            Err(_) => {
                (vec![create_file_info("Failed to load file preview".to_string())], 0)
            }
        }
    } else {
        let contents = get_files_and_dirs(&selected_dir);
        if contents.is_empty() {
            return (vec![create_file_info("empty".to_string())], 0);
        } else {
            return (contents, 0);
        }
    }
}

fn create_file_info(name: String) -> FileInfo {
    FileInfo {
        name,
        perms: None,
        is_dir: false,
        is_exec: false,
    }
}