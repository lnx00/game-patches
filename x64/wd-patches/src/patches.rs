use framework::PatchManager;

pub mod disable_mouse_accel;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_mouse_accel::DisableMouseAccel>();
}
