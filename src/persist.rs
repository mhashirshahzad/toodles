use crate::model::{AppState, PersistedState};
use color_eyre::eyre::Result;
use std::{fs, io::Write, path::PathBuf};

fn state_path() -> Result<PathBuf> {
    let mut path =
        dirs::cache_dir().ok_or_else(|| color_eyre::eyre::eyre!("cannot find cache dir"))?;
    path.push("toodles");
    path.push("todo.toml");
    Ok(path)
}

pub fn load_state(app: &mut AppState) -> Result<()> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(());
    }

    let contents = fs::read_to_string(&path)?;
    let persisted: PersistedState = toml::from_str(&contents)?;
    app.groups = persisted.groups;
    Ok(())
}

pub fn save_state(app: &AppState) -> Result<()> {
    let path = state_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let persisted = PersistedState {
        groups: app.groups.clone(),
    };
    let toml = toml::to_string_pretty(&persisted)?;

    let mut file = fs::File::create(path)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}
