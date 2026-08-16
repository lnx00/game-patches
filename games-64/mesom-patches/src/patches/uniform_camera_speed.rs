use anyhow::{Context, Result, ensure};
use framework::{BytePatch, Patch, utils};

use crate::sdk::offsets;

/*
    The game uses factors 200 (x-axis) and 105 (y-axis) for the camera speed.
    We can force a uniform 1:1 speed by loading the same factor for both axis.
*/

pub struct UniformCameraSpeed {
    byte_patch: BytePatch<8>,
}

impl Patch for UniformCameraSpeed {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Uniform Camera Speed"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("uniform_camera_speed")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let load_x_addr = offsets::LOAD_X_AXIS_FACTOR.get()?;
        let load_y_addr = load_x_addr + 0x8;

        let load_y_inst = unsafe { libmem::disassemble(load_y_addr) }
            .context("failed to disassemble y-factor instruction")?;

        ensure!(
            load_y_inst.mnemonic == "movss",
            "unexpected mnemonic: {}",
            load_y_inst.mnemonic
        );

        let y_displacement = utils::extract_displacement(&load_y_inst)
            .context("failed to extract displacement")? as i32;

        let x_displacement = y_displacement + 0x8;
        let [b0, b1, b2, b3] = x_displacement.to_le_bytes();

        let patch_bytes: [u8; 8] = [
            0xF3, 0x0F, 0x10, 0x0D, // movss xmm1, [rip + ...]
            b0, b1, b2, b3,
        ];

        Ok(Box::new(UniformCameraSpeed {
            byte_patch: BytePatch::new(load_x_addr, patch_bytes),
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
