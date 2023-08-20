extern crate tui;
extern crate crossterm;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use crossterm::terminal;
use tui::widgets::ListState;
use std::env;

mod ui;
use ui::{render_pane, PaneType};
mod fs_utils;
use fs_utils::*;
mod input_handler;
use input_handler::*;
mod preview;
use preview::*;

fn main() {
    // Initialize crossterm
    terminal::enable_raw_mode().unwrap();
    
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut current_dir = env::current_dir().unwrap();
    let mut selected_dir = current_dir.clone();
    let mut scroll_position = 0;

    let mut left_state = ListState::default();
    let mut middle_state = ListState::default();
    let mut right_state = ListState::default();

    middle_state.select(Some(0));

    loop {
        let parents = get_parent_content(&current_dir);
        let files = get_files_and_dirs(&current_dir);

        if let Some(selected) = middle_state.selected() {
            if selected < files.len() {
                let path = current_dir.join(&files[selected].name);
                if files[selected].is_dir {
                    selected_dir = path;
                } else {
                    // selected_dir = path;
                    if selected_dir != path {
                        scroll_position = 0;
                        selected_dir = path;
                    }
                }
            }
        }


        let terminal_size = terminal.size().unwrap();
        let approx_right_pane_height = (terminal_size.height as usize - 4) * 95 / 100;

        let (children, max_scroll) = 
            if selected_dir.as_os_str().is_empty() {
                (vec![FileInfo {
                    name: "Select a directory or file".to_string(),
                    perms: None,
                    is_dir: false,
                    is_exec: false
                }], 0)
            } else if selected_dir.is_file() {
                match get_file_preview(&selected_dir, scroll_position, approx_right_pane_height) {
                    Ok((preview_text, max_scroll_position)) => {
                        (vec![FileInfo {
                            name: preview_text,
                            perms: None,
                            is_dir: false,
                            is_exec: false
                        }], max_scroll_position)
                    },
                    Err(_) => {
                        (vec![FileInfo {
                            name: "Failed to load file preview".to_string(),
                            perms: None,
                            is_dir: false,
                            is_exec: false
                        }], 0)
                    }
                }
            } else {
                let contents = get_files_and_dirs(&selected_dir);
                if contents.is_empty() {
                    (vec![FileInfo {
                        name: "empty".to_string(),
                        perms: None,
                        is_dir: false,
                        is_exec: false,
                    }], 0)
                } else {
                    (contents, 0)
                }
            };

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            render_pane(f, chunks[0], &parents, &mut left_state, PaneType::Left);
            render_pane(f, chunks[1], &files, &mut middle_state, PaneType::Middle);
            render_pane(f, chunks[2], &children, &mut right_state, PaneType::Right);
        }).unwrap();

        // Handle input
        if handle_input(&mut current_dir, &mut selected_dir, &mut middle_state, &mut left_state, &files, &mut scroll_position, &max_scroll) {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
}
