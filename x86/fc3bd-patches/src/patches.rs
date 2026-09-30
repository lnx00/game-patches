use framework::PatchManager;

pub mod disable_input_clamp;
pub mod sensitivity_fix;
pub mod cpu_affinity_fix;

pub fn register_all(manager: &mut PatchManager) {
    manager.register::<disable_input_clamp::DisableInputClamp>();
    manager.register::<sensitivity_fix::SensitivityFix>();
    manager.register::<cpu_affinity_fix::CpuAffinityFix>();
}
