use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::utils;

use super::Patch;

/*
    The game already has logic for disabling camera smoothing, but it is usually not possible
    to enable it. We can patch the condition that checks if mouse smoothing is disabled to
    always run, which causes the game to use the mouse movement directly instead of lerping it.
*/

pub struct DisableCameraSmoothing {
    target_address: usize,
    original_bytes: Box<[u8; 2]>,
}

static INSTANCE: LazyLock<Mutex<DisableCameraSmoothing>> = LazyLock::new(|| {
    let game_module = libmem::find_module("ACU.exe").unwrap();
    let target_address = unsafe {
        libmem::sig_scan("74 ? 41 8B 06 41 89 85", game_module.base, game_module.size)
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

    fn apply(&mut self) -> Result<(), String> {
        let patch_bytes: [u8; 2] = [0x90, 0x90];
        utils::patch_bytes(self.target_address, &patch_bytes)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes(self.target_address, self.original_bytes.as_slice())?;

        Ok(())
    }
}
