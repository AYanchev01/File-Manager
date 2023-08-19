use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::layout::Rect;
use tui::style::{self, Color};
use super::fs_utils;
use super::fs_utils::FileInfo;

#[derive(PartialEq)]
pub enum PaneType {
    Left,
    Middle,
    Right,
}

pub fn render_pane(
    f: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    chunk: Rect,
    items: &[FileInfo],
    state: &mut ListState,
    pane_type: PaneType,
) {
    let list_items: Vec<ListItem> = items.iter().enumerate().map(|(index, file_info)| {
        let item_content = match pane_type {
            PaneType::Middle => {
                if let Some(permissions) = &file_info.perms {
                    let perms_str = fs_utils::get_permissions(permissions);
                    format!("{:<width$} {}", file_info.name, perms_str, width = chunk.width as usize - perms_str.len() - 4)
                } else {
                    file_info.name.clone()
                }
            }
            _ => file_info.name.clone(),
        };

        // Style for the normal, non-selected state
        let normal_style = if file_info.is_dir {
            style::Style::default().fg(Color::Blue)
        } else if file_info.is_exec {
            style::Style::default().fg(Color::Green)
        } else {
            style::Style::default().fg(Color::White)
        };

        // Style for when the item is selected
        let selected_style = if file_info.is_dir {
            style::Style::default().fg(Color::Black).bg(Color::Blue)
        } else if file_info.is_exec {
            style::Style::default().fg(Color::Black).bg(Color::Green)
        } else {
            style::Style::default().fg(Color::Black).bg(Color::White)
        };

        // Apply styles
        if Some(index) == state.selected() {
            ListItem::new(item_content).style(selected_style)
        } else {
            ListItem::new(item_content).style(normal_style)
        }
    }).collect();

    let files_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(style::Style::default());

    f.render_stateful_widget(files_list, chunk, state);
}
