use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;
use crate::keymap::{Action, Keymap};

pub fn render(frame: &mut Frame, app: &mut App, keymap: &Keymap) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    for (index, area) in columns.iter().enumerate() {
        let pane = &app.panes[index];
        let active = index == app.active;
        let title = format!(" {} ", pane.path.display());
        let border = if active { Color::Cyan } else { Color::DarkGray };
        let items: Vec<ListItem> = pane
            .entries
            .iter()
            .map(|entry| {
                let suffix = if entry.is_dir {
                    "/"
                } else if entry.is_symlink {
                    "@"
                } else {
                    ""
                };
                ListItem::new(format!("{}{}", entry.name, suffix))
            })
            .collect();
        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border)),
            )
            .highlight_style(
                Style::default()
                    .bg(if active { Color::Blue } else { Color::DarkGray })
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        let mut state = ListState::default().with_selected(if pane.entries.is_empty() {
            None
        } else {
            Some(pane.selected)
        });
        frame.render_stateful_widget(list, *area, &mut state);
    }

    frame.render_widget(
        Paragraph::new(Line::from(format!(
            " {}: switch  {} / {}: move  {}: open  {}: parent  {}: refresh  {}: quit ",
            keymap.label(Action::SwitchPane),
            keymap.label(Action::Up),
            keymap.label(Action::Down),
            keymap.label(Action::Open),
            keymap.label(Action::Parent),
            keymap.label(Action::Refresh),
            keymap.label(Action::Quit),
        ))),
        rows[1],
    );
}
