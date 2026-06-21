pub mod byte_patch;
pub mod config;
pub mod manager;
pub mod patch;
pub mod signature;
pub mod utils;

pub use byte_patch::{BytePatch, BytePatchNt};
pub use config::Config;
pub use manager::PatchManager;
pub use patch::Patch;
pub use signature::LazySignature;
pub use utils::{logging::init_logger, patch_bytes, patch_bytes_nt, sig_scan_module};
