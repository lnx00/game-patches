use crate::framework::manager::PatchManager;

pub mod disable_camera_smoothing;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_camera_smoothing::DisableCameraSmoothing>();
}
