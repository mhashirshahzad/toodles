use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<TodoItem>,
    pub list_state: ratatui::widgets::ListState,
    pub is_add_new: bool,
    pub input_value: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PersistedState {
    pub items: Vec<TodoItem>,
}

pub enum FormAction {
    None,
    Submit,
    Escape,
}
