use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::layout::Rect;
use tui::style::{self, Color};
use std::fs;

pub enum PaneType {
    Left,
    Middle,
    Right,
}

pub fn render_pane(
    f: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    chunk: Rect,
    items: &[(String, Option<fs::Permissions>)],
    state: &mut ListState,
    pane_type: PaneType
) {
    let list_items: Vec<ListItem> = items.iter().map(|(name, permissions)| {
        match pane_type {
            PaneType::Middle => {
                if let Some(permissions) = permissions {
                    let perms_str = format!(
                        "{}{}{}",
                        if permissions.readonly() { "r" } else { "-" },
                        if permissions.readonly() { "w" } else { "-" },
                        if permissions.readonly() { "x" } else { "-" },
                    );
                    ListItem::new(format!("{:<50} {}", name, perms_str))
                } else {
                    ListItem::new(name.to_string())
                }
            }
            _ => ListItem::new(name.to_string())
        }
    }).collect();

    let files_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            style::Style::default()
                .add_modifier(style::Modifier::BOLD)
                .bg(Color::Blue)
        );

    f.render_stateful_widget(files_list, chunk, state);
}
