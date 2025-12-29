use crate::app::App;
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn draw<B: Backend>(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.area());

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|t| {
            let prefix = if t.done { "[x] " } else { "[ ] " };
            ListItem::new(prefix.to_string() + &t.text)
        })
        .collect();

    let mut list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    if !app.adding {
        list = list.block(
            Block::default()
                .title("Todos")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::Green)),
        );
    }
    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let input = if app.adding {
        Paragraph::new(app.input.as_str()).block(
            Block::default()
                .title("Add Todo")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::Green)),
        )
    } else {
        Paragraph::new("a:add  d:delete  enter:toggle  q:quit")
            .block(Block::default().borders(Borders::ALL))
    };

    f.render_widget(input, chunks[1]);
}
