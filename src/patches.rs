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
    fn name() -> &'static str;

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
        let name = <$patch_ty as Patch>::name();
        if CONFIG.patch_enabled(name, true) {
            <$patch_ty as Patch>::inst().apply()?;
            println!("- {} applied", name);
        } else {
            println!("- {} skipped (disabled)", name);
        }
    }};
}

macro_rules! revert_patch {
    ($patch_ty:ty) => {{
        let name = <$patch_ty as Patch>::name();
        if CONFIG.patch_enabled(name, true) {
            <$patch_ty as Patch>::inst().revert()?;
            println!("- {} reverted", name);
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
