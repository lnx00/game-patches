use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::utils;

use super::Patch;

/*
    Just like Assassin's Creed Unity, the game has logic for disabling mouse smoothing.
    We simply need to patch the condition so that the camera target is always applied
    directly without lerping.
*/

pub struct DisableCameraSmoothing {
    target_address: usize,
    original_bytes: Box<[u8; 1]>,
}

static INSTANCE: LazyLock<Mutex<DisableCameraSmoothing>> = LazyLock::new(|| {
    let game_module = libmem::find_module("ACS.exe").unwrap();
    let target_address = unsafe {
        libmem::sig_scan(
            "75 ? 80 7D ? ? 75 ? 48 8B D9",
            game_module.base,
            game_module.size,
        )
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
        let patch_bytes: [u8; 1] = [0xEB];
        utils::patch_bytes_nt(self.target_address, &patch_bytes)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes_nt(self.target_address, self.original_bytes.as_slice())?;

        Ok(())
    }
}
