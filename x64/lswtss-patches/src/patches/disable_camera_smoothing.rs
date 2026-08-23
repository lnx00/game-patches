use crate::sdk::offsets;
use anyhow::{Context, Result, ensure};
use framework::{BytePatch, Patch, utils};

/*
    The game has a maximum limit for camera delta movement and will
    clamp the delta movement beyond that limit. We can disable this
    by skipping the condition that checks if the limit was exceeded.
*/

pub struct DisableCameraSmoothing {
    patch_mounted: BytePatch<8>,
}

impl Patch for DisableCameraSmoothing {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Disable Camera Smoothing"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("disable_camera_smoothing")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let addr_load_decay_rate = offsets::LOAD_DECAY_RATE_ROAMING.get()?;
        let target_addr_mounted = offsets::LOAD_DECAY_RATE_MOUNTED.get()?;

        let load_half_time_inst = unsafe { libmem::disassemble(addr_load_decay_rate) }
            .context("failed to disassemble decay rate load")?;

        ensure!(
            load_half_time_inst.mnemonic == "movss",
            "expected 'movss', found: {}",
            load_half_time_inst.mnemonic
        );

        // GAME_MODULE + 0x521DCD4C
        let decay_rate_abs = utils::resolve_relative_target(&load_half_time_inst)
            .context("failed to extract displacement")?;
        let new_displacement = decay_rate_abs.wrapping_sub(target_addr_mounted + 0x8) as isize;

        let [b0, b1, b2, b3] = i32::try_from(new_displacement)
            .context("distance >= 2GB")?
            .to_le_bytes();

        let bytes_mounted: [u8; 8] = [
            0xF3, 0x0F, 0x10, 0x05, // movss xmm0, [rip + disp32]
            b0, b1, b2, b3,
        ];

        Ok(Box::new(Self {
            patch_mounted: BytePatch::new(target_addr_mounted, bytes_mounted),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.patch_mounted.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.patch_mounted.revert()?;
        Ok(())
    }
}
