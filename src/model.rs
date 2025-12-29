use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoGroup {
    pub name: String,
    pub todos: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppData {
    pub groups: Vec<TodoGroup>,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoGroup {
    pub name: String,
    pub todos: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppData {
    pub groups: Vec<TodoGroup>,
}
