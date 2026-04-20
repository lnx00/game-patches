use std::thread;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

use crate::{config::CONFIG, framework::manager::PatchManager, sdk::GameSdk, utils::platform};

mod config;
mod framework;
mod patches;
mod sdk;
mod utils;

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

const VK_F11: i32 = 0x7A;
const EXPECTED_TIMESTAMP: u32 = 0x5FDE56CF;

fn check_game_version() {
    if let Some(current_timestamp) = platform::get_time_date_stamp() {
        if current_timestamp != EXPECTED_TIMESTAMP {
            tracing::warn!(
                "timestamp mismatch! expected {:#X}, got {:#X}.",
                EXPECTED_TIMESTAMP,
                current_timestamp
            );

            if !CONFIG.suppress_version_mismatch {
                platform::msg_box(
                    "Game version mismatch!\nThe mod may crash or not work correctly.",
                    "Version Mismatch",
                    platform::MsgBoxType::Warning,
                );
            }
        }
    } else {
        tracing::warn!("failed to check game version!");
    }
}

fn run() -> Result<(), String> {
    tracing::info!("waiting for game...");
    sdk::wait_for_game(std::time::Duration::from_secs(15))?;

    tracing::info!("checking game version...");
    check_game_version();

    tracing::info!("initializing sdk...");
    GameSdk::init()?;

    let mut patch_manager = PatchManager::new();

    tracing::info!("initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("applying patches...");
    patch_manager.apply_all();

    if CONFIG.allow_unloading {
        tracing::info!("patches ready! press F11 to unload.");
        while !platform::is_button_down(VK_F11) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        tracing::info!("reverting patches...");
        patch_manager.revert_all();
    } else {
        tracing::info!("patches ready!");
    }

    Ok(())
}

fn main_thread() {
    // Attach console window
    if CONFIG.show_console {
        let title = format!(
            "{} v{} by {}",
            PKG_NAME.unwrap_or("package"),
            PKG_VERSION.unwrap_or("?.?.?"),
            PKG_AUTHORS.unwrap_or("unknown")
        );
        platform::attach_console(&title);
        let _ = enable_ansi_support::enable_ansi_support();
    }

    // Initialize logger
    tracing_subscriber::fmt().pretty().init();

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
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                let _ = DisableThreadLibraryCalls(dll_module.into());
            }
            thread::spawn(main_thread);
        }

        DLL_PROCESS_DETACH => (),

        _ => (),
    }

    true
}
