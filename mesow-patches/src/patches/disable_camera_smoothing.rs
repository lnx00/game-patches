use crate::{
    framework::{byte_patch::BytePatch, patch::Patch},
    sdk::{GameSdk, offsets::sigs},
};

/*
    We can disable mouse smoothing by setting the registers holding
    the smoothing factors for the x- (xmm3) and y-axis (xmm7) to 0.

    This will only affect mouse input, since the gamepad uses a
    different control flow path with custom smoothing.
*/

pub struct DisableCameraSmoothing {
    byte_patch: BytePatch<9>,
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
        let target_address = GameSdk::inst().find_sig(sigs::LOAD_CAMERA_SMOOTHING_FACTORS)?;

        let patch_bytes: [u8; _] = [
            0x66, 0x0F, 0xEF, 0xDB, // pxor xmm3, xmm3
            0x66, 0x0F, 0xEF, 0xFF, // pxor xmm7, xmm7
            0x90, // nop
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
