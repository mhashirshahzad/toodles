use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
}
#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}
fn main() -> Result<()> {
    let mut state = AppState::default();
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
            let to_exit = handle_key_presses(key);
            if to_exit {
                break;
            }
        }
    }
    Ok(())
}

fn handle_key_presses(key: KeyEvent) -> bool {
    match key.code {
        event::KeyCode::Esc => true,
        _ => false,
    }
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    Paragraph::new("Hello!").render(frame.area(), frame.buffer_mut());
}
