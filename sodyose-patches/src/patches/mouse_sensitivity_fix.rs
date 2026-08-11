use std::{arch::naked_asm, ffi::c_void, sync::atomic::AtomicUsize};

use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch, utils};

/*
    Even the lowest in-game sensitivity option is too high for
    modern mice with high DPI. We can fix this issue by multiplying
    the game's sensitivity with a low factor (20%).
*/

static SENS_MULTIPLIER: f32 = 0.01;

static RETURN_ADDR_ROAMING: AtomicUsize = AtomicUsize::new(0);
static RETURN_ADDR_AIMING_Y: AtomicUsize = AtomicUsize::new(0);
static RETURN_ADDR_AIMING_X: AtomicUsize = AtomicUsize::new(0);

const DISPLACEMENT_SIZE_ROAMING: usize = 0x14;
const DISPLACEMENT_SIZE_AIMING: usize = 0x6;

pub struct MouseSensitivityFix {
    byte_patch_roaming: BytePatch<DISPLACEMENT_SIZE_ROAMING>,
    byte_patch_aiming_x: BytePatch<DISPLACEMENT_SIZE_AIMING>,
    byte_patch_aiming_y: BytePatch<DISPLACEMENT_SIZE_AIMING>,
}

impl MouseSensitivityFix {
    #[unsafe(naked)]
    unsafe extern "C" fn hk_mult_delta_time_roaming() {
        naked_asm!(

            // x_turn * mult
            "mulss xmm1, dword ptr [{mult}]",

            // Load turn_speed
            "movss xmm0, dword ptr [edi + 0x4B8]",

            // y_turn * mult
            "mulss xmm2, dword ptr [{mult}]",

            // Jump back
            "jmp dword ptr [{ret}]",

            mult = sym SENS_MULTIPLIER,
            ret = sym RETURN_ADDR_ROAMING,
        );
    }

    #[unsafe(naked)]
    unsafe extern "C" fn hk_mult_delta_time_aiming_x() {
        naked_asm!(
            "mulss xmm0, dword ptr [{mult}]",
            "jmp dword ptr [{ret}]",

            mult = sym SENS_MULTIPLIER,
            ret = sym RETURN_ADDR_AIMING_X,
        );
    }

    #[unsafe(naked)]
    unsafe extern "C" fn hk_mult_delta_time_aiming_y() {
        naked_asm!(
            "mulss xmm0, dword ptr [{mult}]",
            "jmp dword ptr [{ret}]",

            mult = sym SENS_MULTIPLIER,
            ret = sym RETURN_ADDR_AIMING_Y,
        );
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
        let target_addr_roaming = offsets::MULT_DELTA_TIME_ROAMING.get()?;
        let target_addr_aiming_x = offsets::MULT_DELTA_TIME_AIMING_X.get()?;
        let target_addr_aiming_y = offsets::MULT_DELTA_TIME_AIMING_Y.get()?;

        let dest_addr_roaming = Self::hk_mult_delta_time_roaming as *mut c_void as usize;
        let dest_addr_aiming_x = Self::hk_mult_delta_time_aiming_x as *mut c_void as usize;
        let dest_addr_aiming_y = Self::hk_mult_delta_time_aiming_y as *mut c_void as usize;

        RETURN_ADDR_ROAMING.store(
            target_addr_roaming + DISPLACEMENT_SIZE_ROAMING,
            std::sync::atomic::Ordering::Relaxed,
        );
        RETURN_ADDR_AIMING_X.store(
            target_addr_aiming_x + DISPLACEMENT_SIZE_AIMING,
            std::sync::atomic::Ordering::Relaxed,
        );
        RETURN_ADDR_AIMING_Y.store(
            target_addr_aiming_y + DISPLACEMENT_SIZE_AIMING,
            std::sync::atomic::Ordering::Relaxed,
        );

        let patch_bytes_detour_roaming: [u8; _] = {
            let [b0, b1, b2, b3] =
                utils::get_jump_rel32(target_addr_roaming, dest_addr_roaming).to_le_bytes();

            [
                0xE9, b0, b1, b2, b3, // jmp [dest_address]
                0x90, 0x90, 0x90, 0x90, 0x90, // nop
                0x90, 0x90, 0x90, 0x90, 0x90, // nop
                0x90, 0x90, 0x90, 0x90, 0x90, // nop
            ]
        };

        let patch_bytes_detour_aiming_x: [u8; _] = {
            let [b0, b1, b2, b3] =
                utils::get_jump_rel32(target_addr_aiming_x, dest_addr_aiming_x).to_le_bytes();

            [0xE9, b0, b1, b2, b3, 0x90] // jmp [dest_address]; nop
        };

        let patch_bytes_detour_aiming_y: [u8; _] = {
            let [b0, b1, b2, b3] =
                utils::get_jump_rel32(target_addr_aiming_y, dest_addr_aiming_y).to_le_bytes();

            [0xE9, b0, b1, b2, b3, 0x90] // jmp [dest_address]; nop
        };

        Ok(Box::new(Self {
            byte_patch_roaming: BytePatch::new(target_addr_roaming, patch_bytes_detour_roaming),
            byte_patch_aiming_x: BytePatch::new(target_addr_aiming_x, patch_bytes_detour_aiming_x),
            byte_patch_aiming_y: BytePatch::new(target_addr_aiming_y, patch_bytes_detour_aiming_y),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_roaming.apply()?;
        self.byte_patch_aiming_x.apply()?;
        self.byte_patch_aiming_y.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_aiming_y.revert()?;
        self.byte_patch_aiming_x.revert()?;
        self.byte_patch_roaming.revert()?;
        Ok(())
    }
}
