use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub show_console: bool,
    pub allow_unloading: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub patches: HashMap<String, bool>,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    pub fn read(path: impl AsRef<std::path::Path>) -> Option<Config> {
        let contents = fs::read_to_string(path).ok()?;
        let config = toml::from_str(&contents).ok()?;

        Some(config)
    }

    pub fn load_from_exe_dir(relative_path: &str) -> Config {
        let config_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|parent| parent.join(relative_path)))
            .unwrap_or_else(|| std::path::PathBuf::from(relative_path));

        Config::read(&config_path).unwrap_or_default()
    }

    pub fn patch_enabled(&self, name: &str) -> bool {
        self.patches.get(name).cloned().unwrap_or(true)
    }
}
