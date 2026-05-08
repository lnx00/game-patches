use std::sync::LazyLock;

use framework::Config;

const CONFIG_FILE_PATH: &str = "./plugins/mesom_patches.toml";

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.join(CONFIG_FILE_PATH)))
        .unwrap_or_else(|| std::path::PathBuf::from(CONFIG_FILE_PATH));

    Config::read(&config_path).unwrap_or_default()
});
