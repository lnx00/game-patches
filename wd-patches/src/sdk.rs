use std::{thread, time};

use framework::utils;

pub mod offsets;

const GAME_BINARY_TIMESTAMP: u32 = 0x5c07e8eb;

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<(), String> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // VMP paranoia
    thread::sleep(time::Duration::from_secs(5));

    // Check game version
    tracing::info!("checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMP) {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {:#}", e),
    }

    Ok(())
}

pub fn cleanup() -> Result<(), String> {
    Ok(())
}
