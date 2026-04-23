use std::arch::x86_64::__m128;
use std::ffi::c_void;
use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::game::Clock;

use super::Patch;

/*
    We adjust the mouse sensitivity by multiplying the axis movement with a factor, that
    is inversely proportional to the frame time. This will keep the sensitivity constant,
    regardless of the FPS.

    We use the mouse sensitivity at 60 FPS (0.016 ms frame time) as a reference.
*/

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

const ROOT_CLOCK_ADDRESS: usize = 0x14525D9D0;
const GET_AXIS_MOVEMENT_ADDRESS: usize = 0x141F6A320;

const REFERENCE_FRAME_TIME: f32 = 0.016; // 60 FPS

/// Fixes the mouse sensitivity being tied to the FPS.
pub struct MouseSensitivityFix {
    original_func: Option<AxisMovementFn>,
    trampoline: Option<libmem::Trampoline>,
}

static INSTANCE: LazyLock<Mutex<MouseSensitivityFix>> = LazyLock::new(|| {
    Mutex::new(MouseSensitivityFix {
        original_func: None,
        trampoline: None,
    })
});
impl Patch for MouseSensitivityFix {
    fn inst() -> MutexGuard<'static, Self> {
        INSTANCE.lock().unwrap()
    }

    fn apply(&mut self) -> Result<(), String> {
        let original_func: usize = GET_AXIS_MOVEMENT_ADDRESS;
        let hook_func: usize = hk_get_axis_movement as *mut c_void as usize;

        unsafe {
            let trampoline =
                libmem::hook_code(original_func, hook_func).ok_or("failed to hook function")?;

            self.original_func = trampoline.callable();
            self.trampoline = Some(trampoline);
        }

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(GET_AXIS_MOVEMENT_ADDRESS, trampoline);
            }
        }

        Ok(())
    }
}

#[allow(improper_ctypes_definitions)]
extern "fastcall" fn hk_get_axis_movement(
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
        let g_root_clock = &**(ROOT_CLOCK_ADDRESS as *mut *mut Clock);
        let frame_delta_time = g_root_clock.delta_time;

        let new_factor = REFERENCE_FRAME_TIME / frame_delta_time;

        /*println!(
            "frame delta time: {}, sensitivity factor: {}",
            frame_delta_time, new_factor
        );*/

        return MouseSensitivityFix::inst().original_func.unwrap()(
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            invert_factor * new_factor,
            a8,
            a9,
        );
    }
}
