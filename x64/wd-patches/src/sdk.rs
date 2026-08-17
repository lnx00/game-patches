use std::{thread, time};

use anyhow::Result;
use framework::{ResultLogExt, utils::{self, platform}};

pub mod offsets;

const GAME_BINARY_TIMESTAMPS: &[u32] = &[0x5c07e8eb];

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<()> {
    // Wait for game module
    tracing::info!("Waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("Found game module: {}", module);

    // VMP paranoia
    thread::sleep(time::Duration::from_secs(5));

    // Check game version
    tracing::info!("Checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMPS) {
        Ok(version) => tracing::info!("Game version ({:X}) validated", version),
        Err(e) => tracing::warn!("Failed to check game version: {:#}", e),
    }

    // Unhook NtProtectVirtualMemory
    tracing::info!("Unhooking NtProtectVirtualMemory...");
    platform::unhook_prot_memory().warn_and_continue("failed to unhook NtProtectVirtualMemory");

    Ok(())
}
