use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game clamps mouse movement to a maximum of 1.0, which causes a
    negative acceleration effect. There's also a deadzone for small movements,
    causing slow mouse movements to not be registered.

    We can fix these issues by NOP'ing the conditions that clamp/limit the
    mouse movement.
*/

pub struct DisableInputClamp {
    byte_patch_x_limit: BytePatch<8>,
    byte_patch_y_limit: BytePatch<8>,
    byte_patch_deadzone: BytePatch<6>,
}

impl Patch for DisableInputClamp {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Disable Input Clamp"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("disable_input_clamp")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_x_factor = offsets::LIMIT_X_FACTOR_ANCHOR.get()? + 0x41;
        let target_y_factor = offsets::LIMIT_Y_FACTOR_ANCHOR.get()? + 0x41;
        let target_deadzone = offsets::INPUT_DEADZONE_COND.get()?;

        let patch_bytes_factors: [u8; _] = [
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];

        let patch_bytes_deadzone: [u8; _] = [
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // nop
        ];

        Ok(Box::new(Self {
            byte_patch_x_limit: BytePatch::new(target_x_factor, patch_bytes_factors),
            byte_patch_y_limit: BytePatch::new(target_y_factor, patch_bytes_factors),
            byte_patch_deadzone: BytePatch::new(target_deadzone, patch_bytes_deadzone),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_x_limit.apply()?;
        self.byte_patch_y_limit.apply()?;
        self.byte_patch_deadzone.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_deadzone.revert()?;
        self.byte_patch_y_limit.revert()?;
        self.byte_patch_x_limit.revert()?;
        Ok(())
    }
}
