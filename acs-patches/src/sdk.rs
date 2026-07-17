use anyhow::Result;
use framework::utils;

pub mod integrity;
pub mod offsets;

const GAME_BINARY_TIMESTAMP: u32 = 0x6932E389;

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<(), String> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // Check game version
    tracing::info!("checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMP) {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {:#}", e),
    }

    // Handle integrity checks
    tracing::info!("waiting for integrity checks...");
    if let Err(e) = integrity::initialize() {
        tracing::warn!(
            "integrity bypass verification failed: {}. continuing anyway, but the game might crash...",
            e
        );
    }

    Ok(())
}

pub fn cleanup() -> Result<()> {
    tracing::info!("uninstalling integrity hook...");
    integrity::IntegrityHook::inst().cleanup()
}
