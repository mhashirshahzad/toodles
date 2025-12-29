use crate::app::App;
use ratatui::crossterm::event::KeyEvent;
use std::io;

pub fn handle_event(key: KeyEvent, app: &mut App) -> io::Result<bool> {
    if app.adding {
        match key.code {
            ratatui::crossterm::event::KeyCode::Enter => app.add(),
            ratatui::crossterm::event::KeyCode::Esc => {
                app.input.clear();
                app.adding = false;
            }
            ratatui::crossterm::event::KeyCode::Char(c) => app.input.push(c),
            ratatui::crossterm::event::KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        }
        return Ok(false);
    }

    match key.code {
        ratatui::crossterm::event::KeyCode::Char('q') => {
            app.save_app_data();
            Ok(true)
        }
        ratatui::crossterm::event::KeyCode::Char('j')
        | ratatui::crossterm::event::KeyCode::Down => {
            app.next();
            Ok(false)
        }
        ratatui::crossterm::event::KeyCode::Char('k') | ratatui::crossterm::event::KeyCode::Up => {
            app.prev();
            Ok(false)
        }
        ratatui::crossterm::event::KeyCode::Enter => {
            app.toggle();
            Ok(false)
        }
        ratatui::crossterm::event::KeyCode::Char('d') => {
            app.delete();
            Ok(false)
        }
        ratatui::crossterm::event::KeyCode::Char('a') => {
            app.adding = true;
            Ok(false)
        }
        _ => Ok(false),
    }
}
