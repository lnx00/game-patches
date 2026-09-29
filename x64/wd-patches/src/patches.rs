use framework::PatchManager;

pub mod disable_mouse_accel;
pub mod disable_aim_assist;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_mouse_accel::DisableMouseAccel>();
    manager.register::<disable_aim_assist::DisableAimAssist>();
}
