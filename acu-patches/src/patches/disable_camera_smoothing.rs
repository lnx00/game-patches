use crate::{
    framework::{byte_patch::BytePatch, patch::Patch},
    sdk::{GameSdk, offsets::sigs},
};

/*
    The game already has logic for disabling camera smoothing, but it is usually not possible
    to enable it. We can patch the condition that checks if mouse smoothing is disabled to
    always run, which causes the game to use the mouse movement directly instead of lerping it.
*/

pub struct DisableCameraSmoothing {
    byte_patch: BytePatch<2>,
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
        let target_address = GameSdk::inst().find_sig(sigs::JUMP_CAMERA_SMOOTHING)?;

        let patch_bytes: [u8; _] = [
            0x90, 0x90, // nop nop
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
