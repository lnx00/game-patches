pub mod config;
pub mod logging;
pub mod manager;
pub mod patch;
pub mod utils;

pub use config::Config;
pub use logging::init_logger;
pub use manager::PatchManager;
pub use patch::Patch;
pub use utils::{
    byte_patch::BytePatch, byte_patch::BytePatchNt, lazy::LazyModule, lazy::LazySignature,
};

#[macro_export]
macro_rules! dll_main {
    ($entry:expr) => {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        extern "system" fn DllMain(
            dll_module: windows::Win32::Foundation::HINSTANCE,
            call_reason: u32,
            _reserved: *mut (),
        ) -> bool {
            if call_reason == windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
                unsafe {
                    let _ = windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(
                        dll_module.into(),
                    );
                }
                ::std::thread::spawn($entry);
            }

            true
        }
    };
}
