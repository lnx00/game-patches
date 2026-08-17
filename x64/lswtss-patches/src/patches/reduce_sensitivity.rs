use std::{arch::naked_asm, ffi::c_void, sync::atomic::AtomicUsize};

use crate::sdk::offsets;
use anyhow::{Context, Result};
use framework::{BytePatch, Patch, utils};

/*
    Even the lowest in-game sensitivity option is too high for
    modern mice with high DPI. We can fix this issue by multiplying
    the game's sensitivity with a low factor (20%).
*/

static SENS_MULTIPLIER: f32 = 0.2;
static TRAMPOLINE_ADDR: AtomicUsize = AtomicUsize::new(0);

const DISPLACEMENT_SIZE: usize = 0x15;

pub struct ReduceSensitivity {
    byte_patch: BytePatch<DISPLACEMENT_SIZE>,
}

impl ReduceSensitivity {
    #[unsafe(naked)]
    unsafe extern "C" fn hk_mult_sensitivity() {
        naked_asm!(
            "mulss xmm0, dword ptr [rip + {multiplier}]",

            // Restore
            "xor r14d, r14d",
            "mov [rbx + 0x21F0], r14",
            "mov rcx, rbp",
            "movss [rbx + 0x1A3C], xmm0",

            // Return
            "jmp qword ptr [rip + {trampoline}]",

            multiplier = sym SENS_MULTIPLIER,
            trampoline = sym TRAMPOLINE_ADDR,
        );
    }
}

impl Patch for ReduceSensitivity {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Reduce Sensitivity"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("reduce_sensitivity")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_address = offsets::MULT_INPUT_FACTOR.get()?;
        let dest_addr = Self::hk_mult_sensitivity as *mut c_void as usize;

        TRAMPOLINE_ADDR.store(
            target_address + DISPLACEMENT_SIZE,
            std::sync::atomic::Ordering::Relaxed,
        );

        let patch_bytes = utils::create_jmp_patch_far(dest_addr);

        Ok(Box::new(Self {
            byte_patch: BytePatch::new(target_address, patch_bytes),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch.revert()?;
        Ok(())
    }
}
