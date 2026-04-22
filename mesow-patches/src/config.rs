use std::{collections::HashMap, fs, sync::LazyLock};

use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./plugins/mesow_patches.toml";

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.join(CONFIG_FILE_PATH)))
        .unwrap_or_else(|| std::path::PathBuf::from(CONFIG_FILE_PATH));

    Config::read(&config_path).unwrap_or_default()
});

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub show_console: bool,
    pub allow_unloading: bool,
    pub patches: HashMap<String, bool>,
}

impl Config {
    pub fn read(path: impl AsRef<std::path::Path>) -> Option<Config> {
        let contents = fs::read_to_string(path).ok()?;
        let config = toml::from_str(&contents).ok()?;

        Some(config)
    }

    pub fn patch_enabled(&self, name: &str) -> bool {
        self.patches.get(name).cloned().unwrap_or(true)
    }
}
