use crate::framework::manager::PatchManager;

pub mod disable_camera_smoothing;
pub mod mouse_sensitivity_fix;
pub mod uniform_camera_speed;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_camera_smoothing::DisableCameraSmoothing>();
    manager.register::<uniform_camera_speed::UniformCameraSpeed>();
    manager.register::<mouse_sensitivity_fix::MouseSensitivityFix>();
}
