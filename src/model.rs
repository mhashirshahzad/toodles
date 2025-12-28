use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AppState {
    pub groups: Vec<TodoGroup>,
    pub group_state: ListState,
    pub todo_state: ListState,

    pub is_add_group: bool,
    pub is_add_todo: bool,

    pub input_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoGroup {
    pub name: String,
    pub items: Vec<TodoItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PersistedState {
    pub groups: Vec<TodoGroup>,
}
