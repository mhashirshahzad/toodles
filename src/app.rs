use ratatui::widgets::ListState;

use serde::{Deserialize, Serialize};
use std::fs;
use toml;
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppData {
    pub todos: Vec<Todo>,
}
pub struct App {
    pub app_data: AppData,
    pub state: ListState,
    pub input: String,
    pub adding: bool,
}
impl App {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            app_data: AppData { todos: Vec::new() },
            state,
            input: String::new(),
            adding: false,
        }
    }

    pub fn load_or_create_new_app_data() -> Self {
        let mut file = dirs::cache_dir().unwrap(); // get ~/.cache
        file.push("toodles"); // add directory
        std::fs::create_dir_all(&file).unwrap(); // ensure it exists
        file.push("todo.toml"); // add file name           
        if file.exists() {
            Self::load_app_data()
        } else {
            Self::new()
        }
    }
    pub fn load_app_data() -> Self {
        let mut file = dirs::cache_dir().unwrap();
        file.push("toodles");
        std::fs::create_dir_all(&file).unwrap();
        file.push("todo.toml");

        let app_data: AppData = if file.exists() {
            let content =
                fs::read_to_string(&file).expect("[Error] Reading ~/.cache/toodles/todo.toml");
            toml::from_str(&content).unwrap_or_default()
        } else {
            AppData { todos: Vec::new() }
        };

        let mut state = ListState::default();
        if !app_data.todos.is_empty() {
            state.select(Some(0));
        }

        Self {
            app_data,
            state,
            input: String::new(),
            adding: false,
        }
    }
    pub fn save_app_data(&self) {
        let mut file = dirs::cache_dir().unwrap();
        file.push("toodles");
        std::fs::create_dir_all(&file).unwrap();
        file.push("todo.toml");

        let toml_string = toml::to_string(&self.app_data).expect("Failed to serialize");

        fs::write(file, toml_string).expect("Failed to write todo file");
    }

    pub fn next(&mut self) {
        if self.app_data.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) if i + 1 < self.app_data.todos.len() => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.app_data.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(0) | None => self.app_data.todos.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }

    pub fn toggle(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(todo) = self.app_data.todos.get_mut(i) {
                todo.done = !todo.done;
            }
        }
    }

    pub fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            if i < self.app_data.todos.len() {
                self.app_data.todos.remove(i);
            }
        }
        if self.app_data.todos.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn add(&mut self) {
        if !self.input.trim().is_empty() {
            self.app_data.todos.push(Todo {
                text: self.input.trim().to_string(),
                done: false,
            });
        }
        self.input.clear();
        self.adding = false;
        self.state
            .select(Some(self.app_data.todos.len().saturating_sub(1)));
    }
}
