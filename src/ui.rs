use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Color,
    style::{Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, Widget},
};

use crate::model::AppState;

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded)
                    .title(" Add New Todo ".to_span().into_centered_line()),
            )
            .render(border_area, frame.buffer_mut());
    } else {
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .title(" Toodles ".to_span().into_centered_line())
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        let list = List::new(app_state.items.iter().map(|item| {
            let span = if item.is_done {
                item.description.to_span().crossed_out()
            } else {
                item.description.to_span()
            };
            ListItem::from(span)
        }))
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
