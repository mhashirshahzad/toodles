use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent},
        style::Color,
    },
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;
        // Input Handling
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                    }
                    FormAction::Submit => {
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                        app_state.is_add_new = false;
                    }
                    FormAction::None => {}
                }
            } else {
                let to_exit = handle_key_presses(key, app_state);
                if to_exit {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Esc => return FormAction::Escape,
        event::KeyCode::Enter => return FormAction::Submit,
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        _ => {}
    }

    FormAction::None
}

fn handle_key_presses(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => true,
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(index) {
                    item.is_done = !item.is_done;
                }
                false
            } else {
                false
            }
        }
        event::KeyCode::Up => {
            app_state.list_state.select_previous();
            false
        }
        event::KeyCode::Down => {
            app_state.list_state.select_next();
            false
        }
        event::KeyCode::Char(char) => match char {
            'j' => {
                app_state.list_state.select_next();
                false
            }
            'k' => {
                app_state.list_state.select_previous();
                false
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                    app_state.list_state.select_previous();
                    false
                } else {
                    false
                }
            }
            'n' => {
                app_state.is_add_new = true;
                false
            }
            'q' => true,
            _ => false,
        },
        _ => false,
    }
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
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
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        let list = List::new(app_state.items.iter().map(|x| {
            let value = if x.is_done {
                x.description.to_span().crossed_out()
            } else {
                x.description.to_span()
            };
            ListItem::from(value)
        }))
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(ratatui::style::Color::Green));
        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
