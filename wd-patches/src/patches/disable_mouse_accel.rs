use crate::sdk::offsets;
use framework::{BytePatchNt, Patch};
use anyhow::Result;

/*
    The game applies mouse acceleration. We can disable this by skipping
    the corresponding function call.

    There is also a maximum limit for the camera delta movement. This
    can be disabled by patching out the condition that checks the
    delta vector magnitude.
*/

pub struct DisableMouseAccel {
    byte_patch_accel: BytePatchNt<5>,
    byte_patch_clamp: BytePatchNt<6>,
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

    fn init() -> Result<Box<dyn Patch>, String>
    where
        Self: Sized,
    {
        let target_address_accel = offsets::CALL_MOUSE_ACCELERATION
            .get()
            .map_err(|_| "CALL_MOUSE_ACCELERATION")?;
        let target_address_clamp = offsets::CLAMP_INPUT_CONDITION
            .get()
            .map_err(|_| "CLAMP_INPUT_CONDITION")?;

        let patch_bytes_accel: [u8; _] = [
            0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];

        // TODO: Extract the jump target dynamically
        let patch_bytes_clamp: [u8; _] = [
            0xE9, 0x93, 0x00, 0x00, 0x00, // jmp 0x93
            0x90, // nop
        ];

        Ok(Box::new(Self {
            byte_patch_accel: BytePatchNt::new(target_address_accel, patch_bytes_accel),
            byte_patch_clamp: BytePatchNt::new(target_address_clamp, patch_bytes_clamp),
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
