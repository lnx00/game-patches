mod plugin;

use crate::{config::CONFIG, patches};
use anyhow::Result;
use framework::PatchManager;
use plugin::{ACUPluginInfo, ACUPluginLoaderInterface, PLUGIN_API_VERSION, make_version};
use windows::Win32::Foundation::HINSTANCE;

fn run() -> Result<()> {
    let mut patch_manager = PatchManager::new();

    tracing::info!("Initializing patches...");
    patches::register_all(&mut patch_manager);

    tracing::info!("Applying patches...");
    patch_manager.apply_all(&CONFIG);

    tracing::info!("Patches ready!");

    Ok(())
}

extern "C" fn init_patches(_plugin_loader: &ACUPluginLoaderInterface) -> bool {
    if let Err(e) = run() {
        tracing::error!("Fatal error: {:#}", e);
    }

    true
}

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

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HINSTANCE, _call_reason: u32, _: *mut ()) -> bool {
    true
}
