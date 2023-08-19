use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::layout::Rect;
use tui::style::{self, Color};

pub fn render_pane(f: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>, chunk: Rect, items: &[String], state: &mut ListState) {
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(i.as_str())).collect();
    let files_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            style::Style::default()
                .add_modifier(style::Modifier::BOLD)
                .bg(Color::Blue)
        );

    f.render_stateful_widget(files_list, chunk, state);
}
