use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub auto_correct: HashMap<String, String>,
}

pub type History = HashMap<String, u32>;

pub fn get_config_dir() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("cli_corrector");
    fs::create_dir_all(&config_dir).unwrap_or_default();
    config_dir
}

pub fn load_config() -> Config {
    let mut config_path = get_config_dir();
    config_path.push("config.json");

    if let Ok(mut file) = File::open(&config_path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
    }
    Config::default()
}

pub fn save_config(config: &Config) {
    let mut config_path = get_config_dir();
    config_path.push("config.json");
    if let Ok(file) = File::create(config_path) {
        let _ = serde_json::to_writer_pretty(file, config);
    }
}

pub fn load_history() -> History {
    let mut history_path = get_config_dir();
    history_path.push("history.json");

    if let Ok(mut file) = File::open(&history_path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(history) = serde_json::from_str(&contents) {
                return history;
            }
        }
    }
    HashMap::new()
}

pub fn save_history(history: &History) {
    let mut history_path = get_config_dir();
    history_path.push("history.json");
    if let Ok(file) = File::create(history_path) {
        let _ = serde_json::to_writer_pretty(file, history);
    }
}

pub fn update_history(history: &mut History, mistyped: &str, suggested: &str) -> u32 {
    let key = format!("{} -> {}", mistyped, suggested);
    let count = {
        let entry = history.entry(key).or_insert(0);
        *entry += 1;
        *entry
    };
    save_history(history);
    count
}
