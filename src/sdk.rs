use std::{sync::OnceLock, thread};

use libmem::Module;

pub mod offsets;

static SDK_INSTANCE: OnceLock<GameSdk> = OnceLock::new();

pub struct GameSdk {
    pub game_module: Module
}

unsafe impl Send for GameSdk {}
unsafe impl Sync for GameSdk {}

impl GameSdk {
    pub fn init() -> Result<(), String> {
        let game_module = libmem::find_module("ShadowOfMordor.exe").unwrap();

        let sdk = GameSdk {
            game_module
        };

        SDK_INSTANCE.set(sdk).map_err(|_| "SDK already initialized")?;

        Ok(())
    }

    pub fn inst() -> &'static GameSdk {
        SDK_INSTANCE.get().expect("SDK was accessed before initialization")
    }
}

pub fn wait_for_game() {
    thread::sleep(std::time::Duration::from_secs(5));
}