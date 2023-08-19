extern crate tui;
extern crate crossterm;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use crossterm::terminal;
use tui::widgets::ListState;
use std::path::PathBuf;
use std::env;

mod ui;
use ui::{render_pane, PaneType};
mod fs_utils;
use fs_utils::*;
mod input_handler;
use input_handler::*;

fn main() {
    // Initialize crossterm
    terminal::enable_raw_mode().unwrap();
    
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut current_dir = env::current_dir().unwrap();
    let mut selected_dir = current_dir.clone();

    let mut left_state = ListState::default();
    let mut middle_state = ListState::default();
    let mut right_state = ListState::default();

    middle_state.select(Some(0));

    loop {
        let parents = get_parent_content(&current_dir);
        let files = get_files_and_dirs(&current_dir);

        if let Some(selected) = middle_state.selected() {
            if selected < files.len() {
                let path = current_dir.join(&files[selected].0);
                if path.is_dir() {
                    selected_dir = path; 
                } else {
                    selected_dir = PathBuf::new();  // Empty pathbuf to indicate it's a file
                }
            }
        }

        let children = if selected_dir.as_os_str().is_empty() {
            // if selected_dir is empty, it means the selection is a file
            vec![("contents of file".to_string(), None, false)]
        } else {
            let contents = get_files_and_dirs(&selected_dir);
            if contents.is_empty() {
                vec![("empty".to_string(), None, false)]
            } else {
                contents
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
        if handle_input(&mut current_dir, &mut selected_dir, &mut middle_state, &mut left_state, &files) {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
}
