use anyhow::Result;
use framework::utils;

pub mod integrity;
pub mod offsets;

const GAME_BINARY_TIMESTAMPS: &[u32] = &[0x6932E389];

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<()> {
    // Wait for game module
    tracing::info!("Waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("Found game module: {}", module);

    // Check game version
    tracing::info!("Checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMPS) {
        Ok(version) => tracing::info!("Game version ({:#x}) validated", version),
        Err(e) => tracing::warn!("Failed to check game version: {:#}", e),
    }

    // Handle integrity checks
    tracing::info!("Waiting for integrity checks...");
    if let Err(e) = integrity::initialize() {
        tracing::warn!(
            "Integrity bypass verification failed: {:#}. Continuing anyway, but the game might crash...",
            e
        );
    }

    Ok(())
}

pub fn cleanup() -> Result<()> {
    tracing::info!("Uninstalling integrity hook...");
    integrity::IntegrityHook::inst().cleanup()
}
