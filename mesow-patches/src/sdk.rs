use std::{thread, time::Duration};

use framework::utils;

pub mod offsets;

const GAME_BINARY_TIMESTAMPS: &[u32] = &[0x5B7F5782];

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<(), String> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // Paranoia wait
    thread::sleep(Duration::from_secs(5));

    // Check game version
    tracing::info!("checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMPS) {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {:#}", e),
    }

    Ok(())
}

pub fn cleanup() -> Result<(), String> {
    Ok(())
}
