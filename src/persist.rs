use std::{fs, io::Write, path::Path};

use color_eyre::eyre::{Result, WrapErr};

use crate::model::{AppState, PersistedState};

const STATE_PATH: &str = "~/.cache/toodles/todo.toml";

pub fn load_state(app: &mut AppState) -> Result<()> {
    let path = Path::new(STATE_PATH);

    if !path.exists() {
        return Ok(());
    }

    let contents = fs::read_to_string(path).wrap_err("failed to read todo state")?;

    let persisted: PersistedState =
        toml::from_str(&contents).wrap_err("failed to parse todo state")?;

    app.items = persisted.items;

    Ok(())
}

pub fn save_state(app: &AppState) -> Result<()> {
    let path = Path::new(STATE_PATH);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).wrap_err("failed to create cache directory")?;
    }

    let persisted = PersistedState {
        items: app.items.clone(),
    };

    let toml = toml::to_string_pretty(&persisted).wrap_err("failed to serialize todo state")?;

    let mut file = fs::File::create(path).wrap_err("failed to create todo state file")?;

    file.write_all(toml.as_bytes())
        .wrap_err("failed to write todo state")?;

    Ok(())
}
