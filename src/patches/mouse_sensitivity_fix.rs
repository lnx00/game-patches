use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::utils;

use super::Patch;

/*
    (Experimental)

    The camera movement is erroneously multiplied with delta time which
    causes the mouse sensitivity to be tied to the frame rate.
    We try to fix this by multiplying with a constant factor instead.

    But for some reason, getting close to 100 fps will stil cause a massive
    sensitivity reduction...
*/

pub struct MouseSensitivityFix {
    target_address_1: usize,
    target_address_2: usize,

    original_bytes_1: Box<[u8; 5]>,
    original_bytes_2: Box<[u8; 5]>,
}

static INSTANCE: LazyLock<Mutex<MouseSensitivityFix>> = LazyLock::new(|| {
    let game_module = libmem::find_module("ShadowOfMordor.exe").unwrap();

    let target_address_1 = unsafe {
        libmem::sig_scan(
            "F3 41 0F 59 CD F3 0F 58 C1 F3 0F 11 45",
            game_module.base,
            game_module.size,
        )
        .ok_or("signature not found")
        .unwrap()
    };
    let target_address_2 = unsafe {
        libmem::sig_scan(
            "F3 41 0F 59 D5 F3 44 0F 11 5D",
            game_module.base,
            game_module.size,
        )
        .ok_or("signature not found")
        .unwrap()
    };

    let original_bytes_1 = unsafe { libmem::read_memory::<_>(target_address_1) };
    let original_bytes_2 = unsafe { libmem::read_memory::<_>(target_address_2) };

    Mutex::new(MouseSensitivityFix {
        target_address_1,
        target_address_2,
        original_bytes_1: Box::new(original_bytes_1),
        original_bytes_2: Box::new(original_bytes_2),
    })
});

impl Patch for MouseSensitivityFix {
    fn inst() -> MutexGuard<'static, Self> {
        INSTANCE.lock().unwrap()
    }

    fn apply(&mut self) -> Result<(), String> {
        let patch_bytes_1: [u8; 5] = [0xF3, 0x41, 0x0F, 0x59, 0xCA]; // mulss xmm1, xmm10
        let patch_bytes_2: [u8; 5] = [0xF3, 0x41, 0x0F, 0x59, 0xD2]; // mulss xmm2, xmm10

        utils::patch_bytes(self.target_address_1, &patch_bytes_1)?;
        utils::patch_bytes(self.target_address_2, &patch_bytes_2)?;

        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        utils::patch_bytes(self.target_address_1, self.original_bytes_1.as_slice())?;
        utils::patch_bytes(self.target_address_2, self.original_bytes_2.as_slice())?;

        Ok(())
    }
}
