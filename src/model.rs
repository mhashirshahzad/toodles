use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AppState {
    pub groups: Vec<TodoGroup>,
    pub group_state: ListState,
    pub todo_state: ListState,
    pub input_value: String,
    pub focus: Focus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Groups,
    Todos,
    AddGroup,
    AddTodo,
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
