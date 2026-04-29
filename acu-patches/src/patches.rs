use crate::framework::manager::PatchManager;

pub mod disable_camera_smoothing;
pub mod mouse_sensitivity_fix;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_camera_smoothing::DisableCameraSmoothing>();
    manager.register::<mouse_sensitivity_fix::MouseSensitivityFix>();
}
