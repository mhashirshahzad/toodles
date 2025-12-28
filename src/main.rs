use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

mod app;
mod event;
mod model;
mod persist;
mod ui;

use app::run;
use model::AppState;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut state = AppState::default();
    persist::load_state(&mut state)?;

    let terminal: DefaultTerminal = ratatui::init();

    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}
