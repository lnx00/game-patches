use anyhow::{Result, bail};
use framework::utils::platform;

pub mod integrity;
pub mod offsets;
pub mod structs;

const GAME_BINARY_TIMESTAMP: u32 = 0x54DB5826;

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<(), String> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // Check game version
    tracing::info!("checking game version...");
    match check_game_version() {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {:#}", e),
    }

    // Handle integrity checks
    tracing::info!("waiting for integrity checks...");
    if let Err(e) = integrity::initialize() {
        tracing::warn!(
            "Integrity bypass verification failed: {:#}. Continuing anyway, but the game might crash...",
            e
        );
    }

    Ok(())
}

pub fn cleanup() -> Result<()> {
    tracing::info!("uninstalling integrity hook...");
    integrity::IntegrityHook::inst().cleanup()
}

pub fn check_game_version() -> Result<u32> {
    if let Some(current_timestamp) = platform::get_time_date_stamp() {
        if current_timestamp != GAME_BINARY_TIMESTAMP {
            bail!(
                "timestamp mismatch (expected {}, got {})",
                GAME_BINARY_TIMESTAMP,
                current_timestamp
            );
        }

        return Ok(current_timestamp);
    }

    bail!("failed to retrieve timestamp")
}
