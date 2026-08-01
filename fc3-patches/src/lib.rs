use std::{sync::RwLock, thread};

use crate::config::CONFIG;
use anyhow::Result;
use framework::{PatchManager, utils::platform};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F10;

mod config;
mod patches;
mod sdk;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

static PATCH_MANAGER: RwLock<Option<PatchManager>> = RwLock::new(None);

/// Tries to clean everything up for safe unloading
fn cleanup() {
    tracing::info!("Reverting patches...");
    if let Some(mut pm) = PATCH_MANAGER.write().unwrap().take() {
        pm.revert_all();
    }

    tracing::info!("Cleanup done!");
}

/// Initializes and runs all patches.
/// Might block the caller, if hotkeys are enabled.
fn run() -> Result<()> {
    sdk::wait_until_ready()?;

    let mut patch_manager = PatchManager::new();

    tracing::info!("Initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("Applying patches...");
    patch_manager.apply_all(&CONFIG);

    *PATCH_MANAGER.write().unwrap() = Some(patch_manager);

    // Wait for unload, if enabled
    if CONFIG.allow_unloading {
        tracing::info!("Patches ready! press F11 to unload.");
        while !platform::is_button_down(VK_F10.0 as i32) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        tracing::info!("F11 pressed! cleaning up...");
        cleanup();
    } else {
        tracing::info!("Patches ready!");
    }

    Ok(())
}

fn main_thread() {
    // Initialize logger
    framework::init_logger(format!("{}.log", PKG_NAME), &CONFIG.log_level);

    // Attach console window
    if CONFIG.show_console {
        let title = format!("{} v{} by {}", PKG_NAME, PKG_VERSION, PKG_AUTHORS);
        platform::attach_console(&title);
        let _ = enable_ansi_support::enable_ansi_support();
        tracing::info!("Running {}", title);
    }

    // Run main logic
    if let Err(e) = run() {
        tracing::error!("Fatal error: {:#}", e);
        platform::msg_box(&format!("{:#}", e), "Error", platform::MsgBoxType::Error);
    }

    // Detach console
    if CONFIG.show_console {
        platform::detach_console();
    }
}

framework::dll_main!(main_thread);
