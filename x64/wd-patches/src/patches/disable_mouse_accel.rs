use crate::sdk::offsets;
use anyhow::Result;
use framework::{BytePatch, Patch};

/*
    The game applies mouse acceleration. We can disable this by skipping
    the corresponding function call.

    There is also a maximum limit for the camera delta movement. This
    can be disabled by patching skipping the conditions that checks the
    delta vector magnitude.
*/

pub struct DisableMouseAccel {
    patch_accel: BytePatch<5>,
    patch_clamp: BytePatch<2>,
    patch_clamp_driving: BytePatch<1>,
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
        let target_addr_accel = offsets::CALL_MOUSE_ACCELERATION.get()?;
        let target_addr_clamp = offsets::CLAMP_INPUT_CONDITION.get()?;
        let target_addr_clamp_driving = offsets::APPLY_DRIVING_DEADZONE_COND.get()?;

        let bytes_accel: [u8; _] = [0x90; 5]; // nop

        let bytes_clamp: [u8; _] = [
            0x90, // nop
            0xE9, // jmp
        ];

        let bytes_clamp_driving: [u8; _] = [0xEB]; // jmp

        Ok(Box::new(Self {
            patch_accel: BytePatch::new(target_addr_accel, bytes_accel),
            patch_clamp: BytePatch::new(target_addr_clamp, bytes_clamp),
            patch_clamp_driving: BytePatch::new(target_addr_clamp_driving, bytes_clamp_driving),
        }))
    }

    fn apply(&mut self) -> Result<()> {
        self.patch_accel.apply()?;
        self.patch_clamp.apply()?;
        self.patch_clamp_driving.apply()?;
        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        self.patch_clamp_driving.revert()?;
        self.patch_clamp.revert()?;
        self.patch_accel.revert()?;
        Ok(())
    }
}
