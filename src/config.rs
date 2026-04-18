use std::{collections::HashMap, fs, sync::LazyLock};

use serde::Deserialize;

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| match Config::read("./plugins/mesom_patches.toml") {
        Some(config) => config,
        None => {
            tracing::warn!("failed to load config file! using default config instead.");
            Config::default()
        }
    });

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub show_console: bool,
    pub allow_unloading: bool,
    pub suppress_version_mismatch: bool,
    pub patches: HashMap<String, bool>,
}

impl Config {
    pub fn read(path: &str) -> Option<Config> {
        let contents = fs::read_to_string(path).ok()?;
        let config = toml::from_str(&contents).ok()?;

        Some(config)
    }

    pub fn patch_enabled(&self, name: &str) -> bool {
        self.patches.get(name).cloned().unwrap_or(true)
    }
}
