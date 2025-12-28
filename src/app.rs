use crate::{
    event::{handle_add_new, handle_key_presses},
    model::AppState,
    persist::save_state,
    ui::render,
};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event};

pub fn run(
    mut terminal: DefaultTerminal,
    app_state: &mut AppState,
) -> color_eyre::eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_add_group || app_state.is_add_todo {
                handle_add_new(key, app_state); // returns bool, but you can ignore it
            } else if handle_key_presses(key, app_state) {
                save_state(app_state)?;
                break;
            }
        }
    }
    Ok(())
}
