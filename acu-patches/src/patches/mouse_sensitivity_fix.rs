use anyhow::{Context, Result};
use std::arch::x86_64::__m128;
use std::ffi::c_void;
use std::sync::OnceLock;

use crate::sdk::{offsets, structs};
use framework::{Patch, utils};

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
static ROOT_CLOCK_ADDR: OnceLock<usize> = OnceLock::new();

pub struct MouseSensitivityFix {
    trampoline: Option<libmem::Trampoline>,
    target_address: usize,
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
            let new_factor = ROOT_CLOCK_ADDR
                .get()
                .and_then(|clock_addr| (*clock_addr as *mut *mut structs::Clock).as_ref())
                .and_then(|clock_ptr_ptr| (*clock_ptr_ptr).as_ref())
                .and_then(|clock| {
                    let frame_delta_time = clock.delta_time;
                    if frame_delta_time > f32::EPSILON {
                        Some(REFERENCE_FRAME_TIME / frame_delta_time)
                    } else {
                        None
                    }
                })
                .unwrap_or(1.0);

            ORIG_AXIS_MOVEMENT.wait()(a1, a2, a3, a4, a5, a6, invert_factor * new_factor, a8, a9)
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

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        // Retrieve hook target address
        let call_address = offsets::GET_AXIS_MOVEMENT_CALL.get()?;
        let inst = unsafe { libmem::disassemble(call_address).context("failed to disassemble")? };
        let target_address =
            utils::resolve_relative_target(&inst).context("failed to extract call target")?;

        // Retrieve clock instance
        let sig_address = offsets::ROOT_CLOCK_ACCESS.get()?;
        let inst = unsafe { libmem::disassemble(sig_address).context("failed to disassemble")? };
        let root_clock_address =
            utils::resolve_relative_target(&inst).context("failed to extract root clock address")?;

        let _ = ROOT_CLOCK_ADDR.set(root_clock_address);

        Ok(Box::new(Self {
            trampoline: None,
            target_address,
        }))
    }

    fn apply(&mut self) -> Result<()> {
        let hook_func: usize = Self::hk_get_axis_movement as *mut c_void as usize;

        unsafe {
            let trampoline =
                libmem::hook_code(self.target_address, hook_func).with_context(|| {
                    format!("failed to hook function at {:#x}", self.target_address)
                })?;

            let _ = ORIG_AXIS_MOVEMENT.set(trampoline.callable::<AxisMovementFn>());
            self.trampoline = Some(trampoline);
        }

        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(self.target_address, trampoline);
            }
        }

        Ok(())
    }
}
