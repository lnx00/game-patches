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

        if SDK_INSTANCE.set(sdk).is_err() {
            tracing::warn!("SDK already initialized");
        }

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

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready(timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();

    // Wait for game module
    tracing::info!("waiting for game module...");
    while libmem::find_module(GAME_MODULE_NAME).is_none() {
        if start.elapsed() >= timeout {
            return Err("timeout while waiting for game".to_string());
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }

    // Initialize SDK
    tracing::info!("initializing sdk...");
    GameSdk::init()?;

    // Check game version
    tracing::info!("checking game version...");
    match check_game_version() {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {}", e),
    }

    // Handle integrity checks
    tracing::info!("waiting for integrity checks...");
    if let Err(e) = integrity::initialize(timeout - start.elapsed()) {
        tracing::warn!(
            "integrity bypass verification failed: {}. continuing anyway, but the game might crash...",
            e
        );
    }

    Ok(())
}

pub fn cleanup() -> Result<(), String> {
    tracing::info!("uninstalling integrity hook...");
    integrity::IntegrityHook::inst().cleanup()
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
