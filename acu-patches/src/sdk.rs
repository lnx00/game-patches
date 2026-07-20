use anyhow::Result;
use framework::utils;

pub mod integrity;
pub mod offsets;
pub mod structs;

const GAME_BINARY_TIMESTAMPS: &[u32] = &[0x54DB5826];

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<()> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // Check game version
    tracing::info!("checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMPS) {
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
