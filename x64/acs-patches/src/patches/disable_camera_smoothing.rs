use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    Just like Assassin's Creed Unity, the game has logic for disabling mouse smoothing.
    We simply need to patch the condition so that the camera target is always applied
    directly without lerping.
*/

pub struct DisableCameraSmoothing {
    byte_patch: BytePatch<1>,
}

impl Patch for DisableCameraSmoothing {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Disable Mouse Smoothing"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("disable_camera_smoothing")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_address = offsets::JUMP_CAMERA_SMOOTHING.get()?;

        let patch_bytes: [u8; _] = [
            0xEB, // jmp
        ];

        let byte_patch = BytePatch::new(target_address, patch_bytes);
        Ok(Box::new(Self { byte_patch }))
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
