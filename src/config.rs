use std::{collections::HashMap, fs, sync::LazyLock};

use serde::Deserialize;

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| Config::read("./plugins/mesom_patches.toml").unwrap_or_default());

#[derive(Deserialize)]
pub struct Config {
    pub show_console: bool,
    pub patches: HashMap<String, bool>,
}

impl Config {
    pub fn read(path: &str) -> Option<Config> {
        let contents = fs::read_to_string(path).ok()?;
        let config = toml::from_str(&contents).ok()?;

        Some(config)
    }

    pub fn patch_enabled(&self, name: &str, default: bool) -> bool {
        *self.patches.get(name).unwrap_or(&default)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_console: false,
            patches: HashMap::new(),
        }
    }
}
