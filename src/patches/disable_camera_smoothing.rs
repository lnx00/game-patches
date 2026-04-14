use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::utils;

use super::Patch;

/*
    The game loads a smoothing value of 0.05 by default. We can disable
    smoothing by setting the register holding this value (xmm4) to zero.
*/

pub struct DisableCameraSmoothing {
    target_address: usize,
    original_bytes: Box<[u8; 10]>,
}

static INSTANCE: LazyLock<Mutex<DisableCameraSmoothing>> = LazyLock::new(|| {
    let game_module = libmem::find_module("ShadowOfMordor.exe").unwrap();
    let target_address = unsafe {
        libmem::sig_scan("84 C0 75 ? F3 0F 10 25", game_module.base, game_module.size)
            .ok_or("signature not found")
            .unwrap()
    };

    let original_bytes = unsafe { libmem::read_memory::<_>(target_address) };

    Mutex::new(DisableCameraSmoothing {
        target_address,
        original_bytes: Box::new(original_bytes),
    })
});

impl Patch for DisableCameraSmoothing {
    fn inst() -> MutexGuard<'static, Self> {
        INSTANCE.lock().unwrap()
    }

    fn config_key() -> &'static str {
        "disable_camera_smoothing"
    }

    fn apply(&mut self) -> Result<(), String> {
        let patch_bytes: [u8; 10] = [
            0x66, 0x0F, 0xEF, 0xE4, // pxor xmm4, xmm4
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90 // nop
        ];
        utils::patch_bytes(self.target_address, &patch_bytes)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes(self.target_address, self.original_bytes.as_slice())?;

        Ok(())
    }
}
