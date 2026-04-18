use crate::{
    framework::{byte_patch::BytePatch, patch::Patch},
    sdk::{GameSdk, offsets::sigs},
};

/*
    (Experimental)

    The camera movement is erroneously multiplied with delta time which
    causes the mouse sensitivity to be tied to the frame rate.
    We try to fix this by multiplying with a constant factor instead.

    But for some reason, getting close to 100 fps will stil cause a massive
    sensitivity reduction...
*/

pub struct MouseSensitivityFix {
    byte_patch_1: BytePatch<5>,
    byte_patch_2: BytePatch<5>,
}

impl Patch for MouseSensitivityFix {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Mouse Sensitivity Fix"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("mouse_sensitivity_fix")
    }

    fn apply(&mut self) -> Result<(), String> {
        self.byte_patch_1.apply()?;
        self.byte_patch_2.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<(), String> {
        self.byte_patch_1.revert()?;
        self.byte_patch_2.revert()?;
        Ok(())
    }

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let game_module = &GameSdk::inst().game_module;

        let target_address_1 = unsafe {
            libmem::sig_scan(
                sigs::MULT_X_AXIS_DELTA_TIME,
                game_module.base,
                game_module.size,
            )
            .ok_or("signature not found")?
        };

        let target_address_2 = unsafe {
            libmem::sig_scan(
                sigs::MULT_Y_AXIS_DELTA_TIME,
                game_module.base,
                game_module.size,
            )
            .ok_or("signature not found")?
        };

        let patch_bytes_1: [u8; 5] = [0xF3, 0x41, 0x0F, 0x59, 0xCA]; // mulss xmm1, xmm10
        let patch_bytes_2: [u8; 5] = [0xF3, 0x41, 0x0F, 0x59, 0xD2]; // mulss xmm2, xmm10

        let byte_patch_1 = BytePatch::new(target_address_1, patch_bytes_1);
        let byte_patch_2 = BytePatch::new(target_address_2, patch_bytes_2);

        Ok(Box::new(Self {
            byte_patch_1,
            byte_patch_2,
        }))
    }
}
