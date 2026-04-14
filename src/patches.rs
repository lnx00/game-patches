use std::sync::MutexGuard;

use disable_camera_smoothing::DisableCameraSmoothing;
use uniform_camera_speed::UniformCameraSpeed;
use mouse_sensitivity_fix::MouseSensitivityFix;

pub mod disable_camera_smoothing;
pub mod uniform_camera_speed;
pub mod mouse_sensitivity_fix;

pub trait Patch {
    fn inst() -> MutexGuard<'static, Self>;

    fn apply(&mut self) -> Result<(), String>;
    fn revert(&mut self) -> Result<(), String>;
}

pub fn run_all_patches() -> Result<(), String> {
    DisableCameraSmoothing::inst().apply()?;
    println!("- DisableCameraSmoothing applied");

    UniformCameraSpeed::inst().apply()?;
    println!("- UniformCameraSpeed applied");

    MouseSensitivityFix::inst().apply()?;
    println!("- MouseSensitivityFix applied");

    Ok(())
}

pub fn disable_all_patches() -> Result<(), String> {
    DisableCameraSmoothing::inst().revert()?;
    println!("- DisableCameraSmoothing reverted");

    UniformCameraSpeed::inst().revert()?;
    println!("- UniformCameraSpeed reverted");

    MouseSensitivityFix::inst().revert()?;
    println!("- MouseSensitivityFix reverted");

    Ok(())
}
