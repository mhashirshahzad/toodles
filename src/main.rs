use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

#[derive(Debug)]
struct Todo {
    text: String,
    done: bool,
}

struct App {
    todos: Vec<Todo>,
    state: ListState,
    input: String,
    adding: bool,
}

impl App {
    fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            todos: Vec::new(),
            state,
            input: String::new(),
            adding: false,
        }
    }

    fn next(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) if i + 1 < self.todos.len() => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(0) | None => self.todos.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }

    fn toggle(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some(todo) = self.todos.get_mut(i) {
                todo.done = !todo.done;
            }
        }
    }

    fn delete(&mut self) {
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

    fn add(&mut self) {
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

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.adding {
                match key.code {
                    KeyCode::Enter => app.add(),
                    KeyCode::Esc => {
                        app.input.clear();
                        app.adding = false;
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => app.next(),
                KeyCode::Char('k') | KeyCode::Up => app.prev(),
                KeyCode::Enter => app.toggle(),
                KeyCode::Char('d') => app.delete(),
                KeyCode::Char('a') => app.adding = true,
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.area());

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|t| {
            let prefix = if t.done { "[x] " } else { "[ ] " };
            ListItem::new(prefix.to_string() + &t.text)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let input = if app.adding {
        Paragraph::new(app.input.as_str())
            .block(Block::default().title("Add Todo").borders(Borders::ALL))
    } else {
        Paragraph::new("a:add  d:delete  enter:toggle  q:quit")
            .block(Block::default().borders(Borders::ALL))
    };

    f.render_widget(input, chunks[1]);
}

