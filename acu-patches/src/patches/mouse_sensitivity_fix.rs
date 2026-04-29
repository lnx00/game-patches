use std::{arch::x86_64::__m128, ffi::c_void, sync::OnceLock};


use crate::{
    framework::patch::Patch,
    sdk::{offsets::offsets, structs::structs},
};

/*
    We adjust the mouse sensitivity by multiplying the axis movement with a factor, that
    is inversely proportional to the frame time. This will keep the sensitivity constant,
    regardless of the FPS.

    We use the mouse sensitivity at 60 FPS (0.016 ms frame time) as a reference.
*/

#[allow(dead_code)]
#[allow(improper_ctypes_definitions)]
type AxisMovementFn = unsafe extern "system" fn(
    a1: i64,
    a2: i64,
    a3: *mut f32,
    a4: *mut i64,
    a5: *mut i64,
    a6: *mut f32,
    invert_factor: f32,
    a8: f32,
    a9: f32,
) -> __m128;

const REFERENCE_FRAME_TIME: f32 = 0.016;

static ORIG_AXIS_MOVEMENT: OnceLock<AxisMovementFn> = OnceLock::new();

pub struct MouseSensitivityFix {
    trampoline: Option<libmem::Trampoline>,
}

impl MouseSensitivityFix {
    #[allow(improper_ctypes_definitions)]
    extern "system" fn hk_get_axis_movement(
        a1: i64,
        a2: i64,
        a3: *mut f32,
        a4: *mut i64,
        a5: *mut i64,
        a6: *mut f32,
        invert_factor: f32,
        a8: f32,
        a9: f32,
    ) -> __m128 {
        unsafe {
            let g_root_clock = &**(offsets::ROOT_CLOCK_ADDRESS as *mut *mut structs::Clock);
            let frame_delta_time = g_root_clock.delta_time;

            let new_factor = REFERENCE_FRAME_TIME / frame_delta_time;

            /*println!(
                "frame delta time: {}, sensitivity factor: {}",
                frame_delta_time, new_factor
            );*/

            let original = ORIG_AXIS_MOVEMENT
                .get()
                .expect("AxisMovement hook was called but original function is not set");

            original(a1, a2, a3, a4, a5, a6, invert_factor * new_factor, a8, a9)
        }
    }
}

impl Patch for MouseSensitivityFix {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Mouse Sensitivity Fix"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("mouse_sensitivity_fix")
    }

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        Ok(Box::new(Self { trampoline: None }))
    }

    fn apply(&mut self) -> Result<(), String> {
        let original_func: usize = offsets::GET_AXIS_MOVEMENT_ADDRESS;
        let hook_func: usize = Self::hk_get_axis_movement as *mut c_void as usize;

        unsafe {
            let trampoline =
                libmem::hook_code(original_func, hook_func).ok_or("failed to hook function")?;

            let _ = ORIG_AXIS_MOVEMENT.set(trampoline.callable::<AxisMovementFn>());
            self.trampoline = Some(trampoline);
        }

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(offsets::GET_AXIS_MOVEMENT_ADDRESS, trampoline);
            }
        }

        Ok(())
    }
}
