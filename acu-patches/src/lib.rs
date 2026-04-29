use std::{sync::RwLock, thread};
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

use crate::{config::CONFIG, framework::manager::PatchManager, utils::platform};

mod config;
mod framework;
mod patches;
mod sdk;
mod utils;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const VK_F11: i32 = 0x7A;

static PATCH_MANAGER: RwLock<Option<PatchManager>> = RwLock::new(None);

/// Tries to clean everything up for safe unloading
fn cleanup() {
    tracing::info!("reverting patches...");
    if let Some(mut pm) = PATCH_MANAGER.write().unwrap().take() {
        pm.revert_all();
    }

    tracing::info!("cleaning up sdk...");
    if let Err(e) = sdk::cleanup() {
        tracing::error!("failed to cleanup sdk: {}", e);
    }

    tracing::info!("cleanup done!");
}

/// Initializes and runs all patches.
/// Might block the caller, if hotkeys are enabled.
fn run() -> Result<(), String> {
    tracing::info!("waiting for game...");
    sdk::wait_for_game(std::time::Duration::from_secs(30))?;

    tracing::info!("checking game version...");
    match sdk::check_game_version() {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {}", e),
    }

    tracing::info!("initializing sdk...");
    sdk::GameSdk::init()?;

    let mut patch_manager = PatchManager::new();

    tracing::info!("initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("applying patches...");
    patch_manager.apply_all();

    *PATCH_MANAGER.write().unwrap() = Some(patch_manager);

    // Wait for unload, if enabled
    if CONFIG.allow_unloading {
        tracing::info!("patches ready! press F11 to unload.");
        while !platform::is_button_down(VK_F11) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        tracing::info!("F11 pressed! cleaning up...");
        cleanup();
    } else {
        tracing::info!("patches ready!");
    }

    Ok(())
}

fn main_thread() {
    // Initialize logger
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).pretty().init();

    // Attach console window
    if CONFIG.show_console {
        let title = format!("{} v{} by {}", PKG_NAME, PKG_VERSION, PKG_AUTHORS);
        platform::attach_console(&title);
        let _ = enable_ansi_support::enable_ansi_support();
        tracing::info!("running {}", title);
    }

    // Run main logic
    if let Err(e) = run() {
        tracing::error!("Error: {}", e);
        platform::msg_box(&e, "Error", platform::MsgBoxType::Error);
    }

    // Detach console
    if CONFIG.show_console {
        platform::detach_console();
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, reserved: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                let _ = DisableThreadLibraryCalls(dll_module.into());
            }
            thread::spawn(main_thread);
        }

        DLL_PROCESS_DETACH => {
            if reserved.is_null() {
                // This isn't good but we need the lock
                cleanup();
            }
        }

        _ => (),
    }

    true
}
