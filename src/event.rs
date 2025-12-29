use crate::model::{AppState, TodoGroup, TodoItem};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc => {
            app_state.is_add_group = false;
            app_state.is_add_todo = false;
            app_state.input_value.clear();
            app_state.focus = Focus::Groups;
            false
        }

        KeyCode::Enter => {
            match app_state.focus {
                Focus::Groups => {
                    app_state.focus = Focus::Todos;

                    if let Some(group_idx) = app_state.group_state.selected() {
                        let len = app_state.groups[group_idx].items.len();
                        app_state.todo_state.select((len > 0).then_some(0));
                    }
                }

                Focus::AddGroup | Focus::AddTodo => {
                    // submit logic should live here
                    app_state.is_add_group = false;
                    app_state.is_add_todo = false;
                    app_state.input_value.clear();
                    app_state.focus = Focus::Todos;
                }

                _ => {}
            }
            false
        }

        KeyCode::Char(c) => {
            if app_state.is_add_group || app_state.is_add_todo {
                app_state.input_value.push(c);
            }
            false
        }

        KeyCode::Backspace => {
            if app_state.is_add_group || app_state.is_add_todo {
                app_state.input_value.pop();
            }
            false
        }

        _ => false,
    }
}
