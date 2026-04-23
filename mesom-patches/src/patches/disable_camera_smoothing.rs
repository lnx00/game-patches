use crate::{
    framework::{byte_patch::BytePatch, patch::Patch},
    sdk::{GameSdk, offsets::sigs},
};

/*
    The game loads a smoothing value of 0.05 by default. We can disable
    smoothing by setting the register holding this value (xmm4) to zero.
*/

pub struct DisableCameraSmoothing {
    byte_patch: BytePatch<10>,
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

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let target_address = GameSdk::inst().find_sig(sigs::LOAD_CAMERA_SMOOTHING_FACTOR)?;

        let patch_bytes: [u8; _] = [
            0x66, 0x0F, 0xEF, 0xE4, // pxor xmm4, xmm4
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];

        let byte_patch = BytePatch::new(target_address, patch_bytes);
        Ok(Box::new(Self { byte_patch }))
    }

    fn apply(&mut self) -> Result<(), String> {
        self.byte_patch.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        self.byte_patch.revert()?;
        Ok(())
    }
}
