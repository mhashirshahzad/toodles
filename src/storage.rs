use crate::model::AppData;
use dirs::cache_dir;
use std::fs;
use std::path::PathBuf;
use toml;

pub fn get_storage_path() -> PathBuf {
    let mut path = cache_dir().expect("Cannot get cache directory");
    path.push("toodles");
    fs::create_dir_all(&path).expect("Cannot create cache directory");
    path.push("todo.toml");
    path
}

pub fn load() -> AppData {
    let path = get_storage_path();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        AppData::default()
    }
}

pub fn save(data: &AppData) {
    let path = get_storage_path();
    let content = toml::to_string(data).expect("Failed to serialize");
    fs::write(path, content).expect("Failed to write todo file");
}
