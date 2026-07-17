pub mod byte_patch;
pub mod config;
pub mod manager;
pub mod patch;
pub mod lazy;
pub mod utils;

pub use byte_patch::{BytePatch, BytePatchNt};
pub use config::Config;
pub use manager::PatchManager;
pub use patch::Patch;
pub use lazy::{LazyModule, LazySignature};
pub use utils::{logging::init_logger, patch_bytes, patch_bytes_nt};
