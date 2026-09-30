use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game prepares a list of potential aim assist candidates every tick.
    We can disable aim assist by skipping the creation of this list.

    Credits: https://github.com/HRVAT007/wd-rawinput
*/

pub struct DisableAimAssist {
    patch_clamp: BytePatch<3>,
}

impl Patch for DisableAimAssist {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Disable Aim Assist"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("disable_aim_assist")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_addr = offsets::PREPARE_AIM_ASSIST_TARGETS.get()?;

        let bytes: [u8; _] = [
            0x31, 0xC0, // xor eax, eax
            0xC3, // ret
        ];

        Ok(Box::new(Self {
            patch_clamp: BytePatch::new(target_addr, bytes),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.patch_clamp.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.patch_clamp.revert()?;
        Ok(())
    }
}
