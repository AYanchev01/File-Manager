use tui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::Rect,
    style::{Color, Style},
    Frame,
    backend::CrosstermBackend,
};
use std::io::Stdout;

use super::fs_utils::{self, FileInfo};

// Constants for repeated styles
const DIR_COLOR: Color = Color::Blue;
const EXEC_COLOR: Color = Color::Green;
const FILE_COLOR: Color = Color::White;

const SELECTED_BG_COLOR: Color = Color::Black;

#[derive(PartialEq)]
pub enum PaneType {
    Left,
    Middle,
    Right,
}

fn get_style_for_file(file_info: &FileInfo, is_selected: bool) -> Style {
    let (fg, bg) = if file_info.is_dir {
        (DIR_COLOR, SELECTED_BG_COLOR)
    } else if file_info.is_exec {
        (EXEC_COLOR, SELECTED_BG_COLOR)
    } else {
        (FILE_COLOR, SELECTED_BG_COLOR)
    };

    if is_selected {
        Style::default().fg(bg).bg(fg)
    } else {
        Style::default().fg(fg)
    }
}

pub fn render_pane(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    items: &[FileInfo],
    state: &mut ListState,
    pane_type: PaneType,
) {
    let list_items: Vec<ListItem> = items.iter().enumerate().map(|(index, file_info)| {
        let item_content = match pane_type {
            PaneType::Middle if file_info.perms.is_some() => {
                let perms_str = fs_utils::get_permissions(&file_info.perms.as_ref().unwrap());
                format!("{:<width$} {}", file_info.name, perms_str, width = chunk.width as usize - perms_str.len() - 4)
            }
            _ => file_info.name.clone(),
        };

        let is_selected = Some(index) == state.selected();

        let item_style = get_style_for_file(file_info, is_selected);
        ListItem::new(item_content).style(item_style)
    }).collect();

    let files_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default());

    f.render_stateful_widget(files_list, chunk, state);
}
