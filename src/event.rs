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
        KeyCode::Enter => match app_state.focus {
            Focus::Groups => {
                app_state.focus = Focus::Todos;

                if let Some(group_idx) = app_state.group_state.selected() {
                    let len = app_state.groups[group_idx].items.len();
                    if len > 0 {
                        app_state.todo_state.select(Some(0));
                    } else {
                        app_state.todo_state.select(None);
                    }
                }
                false
            }
<<<<<<< HEAD

            Focus::AddGroup | Focus::AddTodo => handle_add_new(key, app_state),

            _ => false,
        },
=======
            app_state.input_value.clear();
            true
        }
>>>>>>> f68f59fe47563c576f8c77db2b9117ca700e65ac
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
            app_state.todo_state.select(Some(0)); // reset todo selection
            false
        }

        KeyCode::Char('l') | KeyCode::Right => {
            app_state.group_state.select_next();
            app_state.todo_state.select(Some(0)); // reset todo selection
            false
        }

        KeyCode::Char('n') => {
            if app_state.group_state.selected().is_some() {
                app_state.is_add_todo = true;
            }
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
            if let (Some(group_idx), Some(todo_idx)) = (
                app_state.group_state.selected(),
                app_state.todo_state.selected(),
            ) {
                let items = &mut app_state.groups[group_idx].items;
                if todo_idx < items.len() {
                    items.remove(todo_idx);
                }

                // fix selection after delete
                if items.is_empty() {
                    app_state.todo_state.select(None);
                } else if todo_idx >= items.len() {
                    app_state.todo_state.select(Some(items.len() - 1));
                }
            }
            false
        }

        _ => false,
    }
}
