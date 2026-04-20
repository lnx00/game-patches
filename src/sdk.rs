use std::{sync::OnceLock, thread, time::Duration};

use libmem::Module;

pub mod offsets;

static SDK_INSTANCE: OnceLock<GameSdk> = OnceLock::new();

pub struct GameSdk {
    pub game_module: Module,
}

unsafe impl Send for GameSdk {}
unsafe impl Sync for GameSdk {}

impl GameSdk {
    pub fn init() -> Result<(), String> {
        let game_module =
            libmem::find_module("ShadowOfMordor.exe").ok_or("game module not found")?;

        tracing::info!(
            "found game module at {:#X} (size: {:#X})",
            game_module.base,
            game_module.size
        );

        let sdk = GameSdk { game_module };

        SDK_INSTANCE
            .set(sdk)
            .map_err(|_| "SDK already initialized")?;

        Ok(())
    }

    pub fn inst() -> &'static GameSdk {
        SDK_INSTANCE
            .get()
            .expect("SDK was accessed before initialization")
    }

    pub fn find_sig(&self, signature: &str) -> Result<usize, String> {
        let result =
            unsafe { libmem::sig_scan(signature, self.game_module.base, self.game_module.size) };

        match result {
            Some(address) => Ok(address),
            None => Err(format!("failed to find signature '{}'", signature)),
        }
    }
}

pub fn wait_for_game(timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();

    while libmem::find_module("ShadowOfMordor.exe").is_none() {
        if start.elapsed() >= timeout {
            return Err("timeout while waiting for game".to_string());
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
