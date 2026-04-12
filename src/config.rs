use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub show_console: bool,
}

impl Config {
    pub fn read(path: &str) -> Option<Config> {
        let contents = fs::read_to_string(path).ok()?;
        let config = toml::from_str(&contents).ok()?;

        Some(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_console: false,
        }
    }
}
