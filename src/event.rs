use crate::model::{AppState, TodoGroup, TodoItem};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc => {
            app_state.is_add_group = false;
            app_state.is_add_todo = false;
            app_state.input_value.clear();
            true
        }
        KeyCode::Enter => {
            if app_state.is_add_group {
                app_state.groups.push(TodoGroup {
                    name: app_state.input_value.clone(),
                    items: vec![],
                });
                app_state.is_add_group = false;
            } else if app_state.is_add_todo {
                if let Some(group_idx) = app_state.group_state.selected() {
                    app_state.groups[group_idx].items.push(TodoItem {
                        is_done: false,
                        description: app_state.input_value.clone(),
                    });
                }
                app_state.is_add_todo = false;
            }
            app_state.input_value.clear();
            true
        }
        KeyCode::Char(c) => {
            app_state.input_value.push(c);
            false
        }
        KeyCode::Backspace => {
            app_state.input_value.pop();
            false
        }
        _ => false,
    }
}

/// Handles normal navigation and commands in the TUI.
/// Returns true if the app should exit.
pub fn handle_key_presses(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => true, // exit
        KeyCode::Char('j') | KeyCode::Down => {
            if app_state.is_add_group || app_state.is_add_todo {
                return false;
            }
            app_state.todo_state.select_next();
            false
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app_state.is_add_group || app_state.is_add_todo {
                return false;
            }
            app_state.todo_state.select_previous();
            false
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app_state.group_state.select_previous();
            false
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app_state.group_state.select_next();
            false
        }
        KeyCode::Char('n') => {
            app_state.is_add_todo = true;
            false
        }
        KeyCode::Char('g') => {
            app_state.is_add_group = true;
            false
        }
        KeyCode::Char('D') => {
            if let Some(i) = app_state.group_state.selected() {
                app_state.groups.remove(i);

                app_state.group_state.select_previous();
            }

            false
        }
        KeyCode::Char('d') => {
            if let Some(i) = app_state.todo_state.selected() {
                app_state.groups.remove(i);
                app_state.group_state.select_previous();
            }

            false
        }
        _ => false,
    }
}
