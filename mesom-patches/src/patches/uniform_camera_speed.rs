use anyhow::Result;
use framework::{BytePatch, Patch};

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
        let target_address = offsets::LOAD_X_AXIS_FACTOR.get()?;

        let patch_bytes: [u8; 8] = [
            0xF3, 0x0F, 0x10, 0x0D, 0x4C, 0xC1, 0x29,
            0x01, // movss xmm1, dword ptr cs:const_flt_105
        ];

        let byte_patch = BytePatch::new(target_address, patch_bytes);

        Ok(Box::new(UniformCameraSpeed { byte_patch }))
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
