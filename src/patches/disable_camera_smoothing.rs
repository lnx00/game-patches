use crate::{
    framework::patch::Patch,
    sdk::{GameSdk, offsets::sigs},
    utils,
};

/*
    The game loads a smoothing value of 0.05 by default. We can disable
    smoothing by setting the register holding this value (xmm4) to zero.
*/

pub struct DisableCameraSmoothing {
    target_address: usize,
    original_bytes: Box<[u8; 10]>,
}

impl Patch for DisableCameraSmoothing {
    fn config_key(&self) -> Option<&'static str> {
        Some("disable_camera_smoothing")
    }

    fn apply(&mut self) -> Result<(), String> {
        let patch_bytes: [u8; 10] = [
            0x66, 0x0F, 0xEF, 0xE4, // pxor xmm4, xmm4
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];
        utils::patch_bytes(self.target_address, &patch_bytes)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes(self.target_address, self.original_bytes.as_slice())?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        return "Disable Mouse Smoothing";
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
            .ok_or("signature not found")
            .unwrap()
        };

        let original_bytes = unsafe { libmem::read_memory::<_>(target_address) };

        Ok(Box::new(Self {
            target_address,
            original_bytes: Box::new(original_bytes),
        }))
    }
}
