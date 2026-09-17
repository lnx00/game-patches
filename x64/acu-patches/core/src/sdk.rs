use std::{thread, time};

use anyhow::Result;
use framework::{
    ResultLogExt,
    utils::{self, platform},
};

pub mod integrity;
pub mod offsets;
pub mod structs;

const GAME_BINARY_TIMESTAMPS: &[u32] = &[0x54DB5826, 0x69945EEF];

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<()> {
    // Wait for game module
    log::info!("Waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    log::info!("Found game module: {}", module);

    // VMP paranoia
    thread::sleep(time::Duration::from_secs(5));

    // Check game version
    log::info!("Checking game version...");
    match utils::check_game_version(GAME_BINARY_TIMESTAMPS) {
        Ok(version) => log::info!("Game version ({:X}) validated", version),
        Err(e) => log::warn!("Failed to check game version: {:#}", e),
    }

    // Handle integrity checks
    log::info!("Waiting for integrity checks...");
    integrity::initialize().warn_and_continue("integrity bypass verification failed");

    // Unhook NtProtectVirtualMemory
    log::info!("Unhooking NtProtectVirtualMemory...");
    platform::unhook_prot_memory().warn_and_continue("failed to unhook NtProtectVirtualMemory");

    Ok(())
}

pub fn cleanup() -> Result<()> {
    log::info!("Uninstalling integrity hook...");
    integrity::IntegrityHook::inst().cleanup()
}
