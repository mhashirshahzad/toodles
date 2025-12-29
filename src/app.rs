use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub done: bool,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub state: ListState,
    pub input: String,
    pub adding: bool,
}

impl App {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            todos: Vec::new(),
            state,
            input: String::new(),
            adding: false,
        }
    }

    pub fn next(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) if i + 1 < self.todos.len() => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(0) | None => self.todos.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }

    pub fn toggle(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(todo) = self.todos.get_mut(i) {
                todo.done = !todo.done;
            }
        }
    }

    pub fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            if i < self.todos.len() {
                self.todos.remove(i);
            }
        }
        if self.todos.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn add(&mut self) {
        if !self.input.trim().is_empty() {
            self.todos.push(Todo {
                text: self.input.trim().to_string(),
                done: false,
            });
        }
        self.input.clear();
        self.adding = false;
        self.state.select(Some(self.todos.len().saturating_sub(1)));
    }
}
