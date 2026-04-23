use std::sync::MutexGuard;

use disable_camera_smoothing::DisableCameraSmoothing;

pub mod disable_camera_smoothing;

pub trait Patch {
    fn inst() -> MutexGuard<'static, Self>;

    fn apply(&mut self) -> Result<(), String>;
    fn revert(&mut self) -> Result<(), String>;
}

pub fn run_all_patches() -> Result<(), String> {
    DisableCameraSmoothing::inst().apply()?;
    println!("- DisableCameraSmoothing applied");

    Ok(())
}

pub fn disable_all_patches() -> Result<(), String> {
    DisableCameraSmoothing::inst().revert()?;
    println!("- DisableCameraSmoothing reverted");

    Ok(())
}
