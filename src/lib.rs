use std::thread;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

use crate::{
    config::CONFIG, framework::{manager::PatchManager, patch::Patch}, game::wait_for_game, patches::{
        disable_camera_smoothing::DisableCameraSmoothing,
        mouse_sensitivity_fix::MouseSensitivityFix, uniform_camera_speed::UniformCameraSpeed,
    }, sdk::GameSdk, utils::platform
};

mod config;
mod framework;
mod game;
mod patches;
mod sdk;
mod utils;

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

const VK_F11: i32 = 0x7A;

fn run(allow_unloading: bool) -> Result<(), String> {
    println!("waiting for game...");
    wait_for_game();

    println!("initializing sdk...");
    GameSdk::init()?;

    let mut patch_manager = PatchManager::new();

    println!("initializing patches...");
    patch_manager.register(DisableCameraSmoothing::init());
    patch_manager.register(UniformCameraSpeed::init());
    patch_manager.register(MouseSensitivityFix::init());

    println!("applying patches...");
    patch_manager.apply_all();

    if allow_unloading {
        println!("patches ready! press F11 to unload.");
        while !platform::is_button_down(VK_F11) {
            thread::sleep(std::time::Duration::from_millis(100));
        }

        println!("reverting patches...");
        patch_manager.revert_all();
    }

    Ok(())
}

fn main_thread() {
    // Attach console window
    if CONFIG.show_console {
        let title = format!(
            "{} {} by {}",
            PKG_NAME.unwrap_or("package"),
            PKG_VERSION.unwrap_or("?.?.?"),
            PKG_AUTHORS.unwrap_or("unknown")
        );
        platform::attach_console(&title);
    }

    // Run main logic
    if let Err(e) = run(CONFIG.allow_unloading) {
        eprintln!("Error: {}", e);
        platform::msg_box(&e, "Error", platform::MsgBoxType::Error);
    }

    // Detach console
    if CONFIG.show_console {
        platform::detach_console();
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            thread::spawn(main_thread);
        }

        DLL_PROCESS_DETACH => (),

        _ => (),
    }

    true
}
