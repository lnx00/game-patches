use std::sync::LazyLock;

use framework::Config;

const CONFIG_FILE_PATH: &str = "./plugins/fc3bd_patches.toml";

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::load_from_exe_dir(CONFIG_FILE_PATH));
