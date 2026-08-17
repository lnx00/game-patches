use std::{arch::naked_asm, ffi::c_void, sync::atomic::AtomicUsize};

use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch, utils};

/*
    The game clamps the pitch between -0.26 and 1.48.
    We adjust the minimum limit by replacing the value in the stack before
    calling the clamping function.
*/

static RETURN_ADDR_LIMIT_MIN: AtomicUsize = AtomicUsize::new(0);

const PITCH_LIMIT_MIN: u32 = (-1.0f32).to_bits();
const DISPLACEMENT_SIZE_LIMIX_MIN: usize = 0x7;

pub struct IncreasePitchLimit {
    byte_patch_limit_min: BytePatch<DISPLACEMENT_SIZE_LIMIX_MIN>,
}

impl IncreasePitchLimit {
    #[unsafe(naked)]
    unsafe extern "C" fn hk_pitch_limit_min() {
        naked_asm!(
            "mov dword ptr [esp + 4], {angle}",
            "jmp dword ptr [{ret}]",

            angle = const PITCH_LIMIT_MIN,
            ret = sym RETURN_ADDR_LIMIT_MIN,
        );
    }
}

impl Patch for IncreasePitchLimit {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Increase Pitch Limit"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("increase_pitch_limit")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_addr_roaming = offsets::LOAD_MIN_PITCH.get()?;

        let dest_addr_roaming = Self::hk_pitch_limit_min as *mut c_void as usize;

        RETURN_ADDR_LIMIT_MIN.store(
            target_addr_roaming + DISPLACEMENT_SIZE_LIMIX_MIN,
            std::sync::atomic::Ordering::Relaxed,
        );

        let patch_bytes_detour_roaming =
            utils::create_jmp_patch_near(target_addr_roaming, dest_addr_roaming);

        Ok(Box::new(Self {
            byte_patch_limit_min: BytePatch::new(target_addr_roaming, patch_bytes_detour_roaming),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_limit_min.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_limit_min.revert()?;
        Ok(())
    }
}
