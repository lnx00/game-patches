use crate::{framework::patch::Patch, utils};

/*
    The game uses factors 200 (x-axis) and 105 (y-axis) for the camera speed.
    We can force a uniform 1:1 speed by loading the same factor for both axis.
*/

pub struct UniformCameraSpeed {
    target_address: usize,
    original_bytes: Box<[u8; 8]>,
}

impl Patch for UniformCameraSpeed {
    fn config_key(&self) -> Option<&'static str> {
        Some("uniform_camera_speed")
    }

    fn apply(&mut self) -> Result<(), String> {
        let patch_bytes: [u8; 8] = [
            0xF3, 0x0F, 0x10, 0x0D, 0x4C, 0xC1, 0x29,
            0x01, // movss xmm1, dword ptr cs:const_flt_105
        ];
        utils::patch_bytes(self.target_address, &patch_bytes)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes(self.target_address, self.original_bytes.as_slice())?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Uniform Camera Speed"
    }

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let game_module = libmem::find_module("ShadowOfMordor.exe").unwrap();
        let target_address = unsafe {
            libmem::sig_scan(
                "F3 0F 10 0D ? ? ? ? F3 0F 10 15 ? ? ? ? EB ? F3 0F 10 54 24",
                game_module.base,
                game_module.size,
            )
            .ok_or("signature not found")
            .unwrap()
        };

        let original_bytes = unsafe { libmem::read_memory::<_>(target_address) };

        Ok(Box::new(UniformCameraSpeed {
            target_address,
            original_bytes: Box::new(original_bytes),
        }))
    }
}
