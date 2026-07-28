use framework::PatchManager;

pub mod disable_input_clamp;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_input_clamp::DisableInputClamp>();
}
