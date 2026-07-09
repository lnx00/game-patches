use std::{thread, time::Duration};

use framework::utils::platform;

pub mod integrity;
pub mod offsets;

const GAME_BINARY_TIMESTAMP: u32 = 0x6932E389;

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready(timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();

    // Wait for game module
    tracing::info!("waiting for game module...");
    offsets::GAME_MODULE.wait();

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
