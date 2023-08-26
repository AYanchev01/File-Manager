extern crate tui;
extern crate crossterm;

use crossterm::terminal;
use crate::state::AppState;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use tui::Terminal;
use tui::widgets::{ListState, Paragraph};
use std::env;

mod ui;
mod fs_utils;
mod input;
mod preview;
mod input_handlers;
mod state;

use ui::{render_pane, PaneType};
use fs_utils::*;
use input::*;

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
            let text_to_display = match (&app_state.prompt_message, &app_state.renaming_buffer, &app_state.creation_buffer, &app_state.permissions_buffer) {
                (Some(prompt), Some(buffer), None, None) => format!("{}{}", prompt, buffer),
                (Some(prompt), None, Some(buffer), None) => format!("{}{}", prompt, buffer),
                (Some(prompt), None, None, Some(buffer)) => format!("{}{}", prompt, buffer),
                (Some(prompt), None, None, None) => prompt.clone(),
                _ => String::new(),
            };

            let text_paragraph = Paragraph::new(text_to_display);
            f.render_widget(text_paragraph, vertical_chunks[1]);
        }).unwrap();

        // Handle input
        if handle_input(&mut current_dir, &mut middle_state, &mut left_state, &files, &mut scroll_position, &max_scroll,&mut app_state) {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
}