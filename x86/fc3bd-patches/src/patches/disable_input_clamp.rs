use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game has a maximum limit for camera delta movement and will
    clamp the delta movement beyond that limit. We can disable this
    by skipping the condition that checks if the limit was exceeded.
*/

pub struct DisableInputClamp {
    byte_patch_clamp: BytePatch<1>,
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
        let target_address_clamp = offsets::CLAMP_INPUT_CONDITION.get()?;

        let patch_bytes_clamp: [u8; _] = [
            0xEB, // jmp
        ];

        Ok(Box::new(Self {
            byte_patch_clamp: BytePatch::new(target_address_clamp, patch_bytes_clamp),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.byte_patch_clamp.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.byte_patch_clamp.revert()?;
        Ok(())
    }
}
