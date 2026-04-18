use crate::{
    framework::patch::Patch,
    sdk::{GameSdk, offsets::sigs},
    utils::byte_patch::BytePatch,
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

    fn apply(&mut self) -> Result<(), String> {
        self.byte_patch.apply()?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        self.byte_patch.revert()?;

        Ok(())
    }

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let game_module = &GameSdk::inst().game_module;
        let target_address = unsafe {
            libmem::sig_scan(
                sigs::LOAD_CAMERA_SMOOTHING_FACTOR,
                game_module.base,
                game_module.size,
            )
            .ok_or("signature not found")?
        };

        let patch_bytes: [u8; _] = [
            0x66, 0x0F, 0xEF, 0xE4, // pxor xmm4, xmm4
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];

        let byte_patch = BytePatch::new(target_address, patch_bytes);

        Ok(Box::new(Self { byte_patch }))
    }
}
