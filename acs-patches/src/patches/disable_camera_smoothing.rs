use crate::{
    framework::{byte_patch::BytePatch, patch::Patch},
    sdk::{GameSdk, offsets::sigs},
};

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

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let target_address = GameSdk::inst().find_sig(sigs::JUMP_CAMERA_SMOOTHING)?;

        let patch_bytes: [u8; _] = [
            0xEB, // jmp
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
