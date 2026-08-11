use framework::PatchManager;

pub mod disable_input_clamp;
pub mod mouse_sensitivity_fix;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_input_clamp::DisableMouseAccel>();
    manager.register::<mouse_sensitivity_fix::MouseSensitivityFix>();
}
