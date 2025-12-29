use crate::model::AppState;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    style::{Color, Style},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph},
};

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .margin(1)
        .split(frame.area());

    // Groups list
    let groups = if app_state.groups.is_empty() {
        List::new([ListItem::from("No groups yet")]).block(
            Block::bordered()
                .title(" Groups ".to_span().into_centered_line())
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Green)),
        )
    } else {
        List::new(
            app_state
                .groups
                .iter()
                .map(|g| ListItem::from(g.name.clone())),
        )
        .block(
            Block::bordered()
                .title(" Groups ".to_span().into_centered_line())
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Green)),
        )
    }
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(groups, chunks[0], &mut app_state.group_state);

    // Todos list
    let todos = if let Some(selected) = app_state.group_state.selected() {
        let group = &app_state.groups[selected];
        if group.items.is_empty() {
            List::new([ListItem::from("No todos yet")]).block(
                Block::bordered()
                    .title(" Todos ".to_span().into_centered_line())
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Green)),
            )
        } else {
            List::new(group.items.iter().map(|t| {
                let span = if t.is_done {
                    t.description.to_span().crossed_out()
                } else {
                    t.description.to_span()
                };
                ListItem::from(span)
            }))
            .block(
                Block::bordered()
                    .title(" Todos ".to_span().into_centered_line())
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Green)),
            )
        }
    } else {
        List::new([ListItem::from("No group selected")]).block(
            Block::bordered()
                .title(" Todos ".to_span().into_centered_line())
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Green)),
        )
    };
    frame.render_stateful_widget(todos, chunks[1], &mut app_state.todo_state);

    // Popup
    if app_state.is_add_group || app_state.is_add_todo {
        let popup_area = centered_rect(60, 20, frame.size());
        let title = if app_state.is_add_group {
            " Add Group "
        } else {
            " Add Todo "
        };
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .title(title.to_span().into_centered_line())
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Green)),
            )
            .render(popup_area, frame.buffer_mut());
    }
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
