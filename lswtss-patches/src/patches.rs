use framework::PatchManager;

pub mod disable_camera_smoothing;
pub mod reduce_sensitivity;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_camera_smoothing::DisableCameraSmoothing>();
    manager.register::<reduce_sensitivity::ReduceSensitivity>();
}
