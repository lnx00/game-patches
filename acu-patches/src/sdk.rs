use std::{sync::OnceLock, thread, time::Duration};

use libmem::Module;

use crate::utils::platform;

pub mod integrity;
pub mod offsets;
pub mod structs;

const GAME_MODULE_NAME: &str = "ACU.exe";
const GAME_BINARY_TIMESTAMP: u32 = 0x54DB5826;

static SDK_INSTANCE: OnceLock<GameSdk> = OnceLock::new();

pub struct GameSdk {
    pub game_module: Module,
}

unsafe impl Send for GameSdk {}
unsafe impl Sync for GameSdk {}

impl GameSdk {
    pub fn init() -> Result<(), String> {
        let game_module = libmem::find_module(GAME_MODULE_NAME).ok_or("game module not found")?;

        tracing::info!(
            "found game module '{}' at {:#X} (size: {:#X})",
            GAME_MODULE_NAME,
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

    /// Finds the signature in the main game module
    pub fn find_sig(&self, signature: &str) -> Result<usize, String> {
        let result =
            unsafe { libmem::sig_scan(signature, self.game_module.base, self.game_module.size) };

        match result {
            Some(address) => Ok(address),
            None => Err(format!("failed to find signature '{}'", signature)),
        }
    }
}

/// Blocks the caller until the game is ready
pub fn wait_for_game(timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();

    while libmem::find_module(GAME_MODULE_NAME).is_none() {
        if start.elapsed() >= timeout {
            return Err("timeout while waiting for game".to_string());
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }

    if !integrity::wait_until_safe(timeout) {
        tracing::warn!(
            "failed to kill integrity checks! continuing in 3 second, but the game will most likely crash..."
        );
        thread::sleep(std::time::Duration::from_secs(3));
    }

    Ok(())
}

pub fn check_game_version() -> Result<u32, String> {
    if let Some(current_timestamp) = platform::get_time_date_stamp() {
        if current_timestamp != GAME_BINARY_TIMESTAMP {
            return Err(format!(
                "timestamp mismatch - expected {}, got {}",
                GAME_BINARY_TIMESTAMP, current_timestamp
            ));
        }

        return Ok(current_timestamp);
    }

    Err("failed to retrieve timestamp".to_string())
}
