mod config;
mod patches;
mod sdk;

#[cfg(feature = "plugin")]
mod plugin;

// --- Standalone mode ---

#[cfg(not(feature = "plugin"))]
use std::{sync::RwLock, thread};

#[cfg(not(feature = "plugin"))]
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

#[cfg(not(feature = "plugin"))]
use framework::{PatchManager, utils::platform};

#[cfg(not(feature = "plugin"))]
use crate::config::CONFIG;

#[cfg(not(feature = "plugin"))]
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
#[cfg(not(feature = "plugin"))]
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(not(feature = "plugin"))]
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[cfg(not(feature = "plugin"))]
const VK_F11: i32 = 0x7A;

#[cfg(not(feature = "plugin"))]
static PATCH_MANAGER: RwLock<Option<PatchManager>> = RwLock::new(None);

/// Tries to clean everything up for safe unloading
#[cfg(not(feature = "plugin"))]
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
#[cfg(not(feature = "plugin"))]
fn run() -> Result<(), String> {
    sdk::wait_until_ready()?;

    let mut patch_manager = PatchManager::new();

    tracing::info!("initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("applying patches...");
    patch_manager.apply_all(&CONFIG);

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

#[cfg(not(feature = "plugin"))]
fn main_thread() {
    // Initialize logger
    framework::init_logger(format!("{}.log", PKG_NAME), &CONFIG.log_level);

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

#[cfg(not(feature = "plugin"))]
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

// --- Plugin mode ---

#[cfg(feature = "plugin")]
use framework::PatchManager;

#[cfg(feature = "plugin")]
use windows::Win32::Foundation::HINSTANCE;

#[cfg(feature = "plugin")]
use plugin::{ACUPluginInfo, ACUPluginLoaderInterface, PLUGIN_API_VERSION, make_version};

#[cfg(feature = "plugin")]
use crate::config::CONFIG;

#[cfg(feature = "plugin")]
fn run() -> Result<(), String> {
    sdk::GameSdk::init()?;

    let mut patch_manager = PatchManager::new();

    tracing::info!("initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("applying patches...");
    patch_manager.apply_all(&CONFIG);

    tracing::info!("patches ready!");

    Ok(())
}

#[cfg(feature = "plugin")]
extern "C" fn init_patches(_plugin_loader: &ACUPluginLoaderInterface) -> bool {
    if let Err(e) = run() {
        tracing::error!("{e}");
    }

    true
}

#[cfg(feature = "plugin")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ACUPluginStart(
    plugin_loader: &ACUPluginLoaderInterface,
    your_plugin_info_out: &mut ACUPluginInfo,
) -> bool {
    let _ = plugin_loader.init_logger();

    tracing::info!(
        "Hello ACUFixes plugin loader version {}",
        plugin_loader.m_plugin_loader_version
    );

    your_plugin_info_out.m_plugin_api_version = PLUGIN_API_VERSION;
    your_plugin_info_out.m_plugin_version = make_version(0, 4, 0, 0);

    your_plugin_info_out.m_init_stage_when_code_patches_are_safe_to_apply = Some(init_patches);

    true
}

#[cfg(feature = "plugin")]
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HINSTANCE, _call_reason: u32, _: *mut ()) -> bool {
    true
}
