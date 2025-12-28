use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::{
    event::{handle_add_new, handle_key_presses},
    model::{AppState, FormAction, TodoItem},
    persist::save_state,
    ui::render,
};

pub fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
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
            } else if handle_key_presses(key, app_state) {
                save_state(app_state)?;
                break;
            }
        }
    }
    Ok(())
}
