use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game applies mouse acceleration. We can disable this by skipping
    the corresponding function call.

    There is also a maximum limit for the camera delta movement. This
    can be disabled by patching skipping the condition that checks the
    delta vector magnitude.
*/

pub struct DisableMouseAccel {
    byte_patch_accel: BytePatch<5>,
    byte_patch_clamp: BytePatch<2>,
}

impl Patch for DisableMouseAccel {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Disable Mouse Acceleration"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("disable_mouse_accel")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_address_accel = offsets::CALL_MOUSE_ACCELERATION.get()?;
        let target_address_clamp = offsets::CLAMP_INPUT_CONDITION.get()?;

        let patch_bytes_accel: [u8; _] = [0x90; 5]; // nop

        let patch_bytes_clamp: [u8; _] = [
            0x90, // nop
            0xE9, // jmp
        ];

        Ok(Box::new(Self {
            byte_patch_accel: BytePatch::new(target_address_accel, patch_bytes_accel),
            byte_patch_clamp: BytePatch::new(target_address_clamp, patch_bytes_clamp),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_accel.apply()?;
        self.byte_patch_clamp.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_clamp.revert()?;
        self.byte_patch_accel.revert()?;
        Ok(())
    }
}
