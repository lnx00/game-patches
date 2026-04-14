use std::sync::MutexGuard;

use disable_camera_smoothing::DisableCameraSmoothing;
use mouse_sensitivity_fix::MouseSensitivityFix;
use uniform_camera_speed::UniformCameraSpeed;

use crate::config::CONFIG;

pub mod disable_camera_smoothing;
pub mod mouse_sensitivity_fix;
pub mod uniform_camera_speed;

pub trait Patch {
    fn inst() -> MutexGuard<'static, Self>;
    fn config_key() -> &'static str;
    fn default_enabled() -> bool {
        true
    }

    fn apply(&mut self) -> Result<(), String>;
    fn revert(&mut self) -> Result<(), String>;
}

macro_rules! patch_types {
    ($m:ident) => {
        $m!(DisableCameraSmoothing);
        $m!(UniformCameraSpeed);
        $m!(MouseSensitivityFix);
    };
}

macro_rules! apply_patch {
    ($patch_ty:ty) => {{
        let config_key = <$patch_ty as Patch>::config_key();
        let default_enabled = <$patch_ty as Patch>::default_enabled();
        if CONFIG.patch_enabled(config_key, default_enabled) {
            <$patch_ty as Patch>::inst().apply()?;
            println!("- {} applied", config_key);
        } else {
            println!("- {} skipped (disabled)", config_key);
        }
    }};
}

macro_rules! revert_patch {
    ($patch_ty:ty) => {{
        let config_key = <$patch_ty as Patch>::config_key();
        if CONFIG.patch_enabled(config_key, true) {
            <$patch_ty as Patch>::inst().revert()?;
            println!("- {} reverted", config_key);
        }
    }};
}

pub fn run_all_patches() -> Result<(), String> {
    patch_types!(apply_patch);
    Ok(())
}

pub fn disable_all_patches() -> Result<(), String> {
    patch_types!(revert_patch);
    Ok(())
}
