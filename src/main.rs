use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent},
        terminal,
    },
};
fn main() -> Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering
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
