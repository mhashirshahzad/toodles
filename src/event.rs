use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

use crate::model::{AppState, FormAction, TodoItem};

pub fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        KeyCode::Esc => FormAction::Escape,
        KeyCode::Enter => FormAction::Submit,
        KeyCode::Char(c) => {
            app_state.input_value.push(c);
            FormAction::None
        }
        KeyCode::Backspace => {
            app_state.input_value.pop();
            FormAction::None
        }
        _ => FormAction::None,
    }
}

pub fn handle_key_presses(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => true,

        KeyCode::Enter => {
            if let Some(i) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(i) {
                    item.is_done = !item.is_done;
                }
            }
            false
        }

        KeyCode::Up | KeyCode::Char('k') => {
            app_state.list_state.select_previous();
            false
        }

        KeyCode::Down | KeyCode::Char('j') => {
            app_state.list_state.select_next();
            false
        }

        KeyCode::Char('D') => {
            if let Some(i) = app_state.list_state.selected() {
                app_state.items.remove(i);
                app_state.list_state.select_previous();
            }
            false
        }

        KeyCode::Char('n') => {
            app_state.is_add_new = true;
            false
        }

        _ => false,
    }
}
