use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game has a maximum limit for camera delta movement and will
    clamp the delta movement beyond that limit. We can disable this
    by skipping the condition that checks if the limit was exceeded.
*/

pub struct DisableCameraSmoothing {
    byte_patch_mounted: BytePatch<8>,
    byte_patch_roaming: BytePatch<1>,
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
        let target_address_mounted_1 = offsets::LOAD_DECAY_RATE_MOUNTED.get()?;
        //let target_address_mounted_2 = target_address_mounted_1 + 0xC;

        let target_address_roaming = offsets::SMOOTHING_FALLBACK_COND_ROAMING.get()?;

        let patch_bytes_mounted_1: [u8; _] = [
            0x66, 0x0F, 0xEF, 0xC0, // pxor xmm0, xmm0
            0x90, 0x90, 0x90, 0x90, // nop
        ];

        let patch_bytes_roaming: [u8; _] = [
            0xEB, // jmp
        ];

        Ok(Box::new(Self {
            byte_patch_mounted: BytePatch::new(target_address_mounted_1, patch_bytes_mounted_1),
            byte_patch_roaming: BytePatch::new(target_address_roaming, patch_bytes_roaming),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_mounted.apply()?;
        self.byte_patch_roaming.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_roaming.revert()?;
        self.byte_patch_mounted.revert()?;
        Ok(())
    }
}
