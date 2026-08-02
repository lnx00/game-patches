use std::{arch::naked_asm, ffi::c_void, sync::atomic::AtomicUsize};

use crate::sdk::offsets;
use anyhow::{Context, Result};
use framework::Patch;

static SENS_MULTIPLIER: f32 = 0.2;
static TRAMPOLINE_ADDR: AtomicUsize = AtomicUsize::new(0);

pub struct SensitivityFix {
    trampoline: Option<libmem::Trampoline>,
    target_address: usize,
}

impl SensitivityFix {
    #[unsafe(naked)]
    unsafe extern "C" fn hk_mult_sensitivity() {
        naked_asm!(
            "mulss xmm0, dword ptr [{multiplier}]",
            "jmp dword ptr [{trampoline}]",

            multiplier = sym SENS_MULTIPLIER,
            trampoline = sym TRAMPOLINE_ADDR,
        );
    }
}

impl Patch for SensitivityFix {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Sensitivity Fix"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("sensitivity_fix")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let target_address = offsets::LOAD_SENSITIVITY.get()? + 0x5;

        Ok(Box::new(Self {
            trampoline: None,
            target_address,
        }))
    }

    fn apply(&mut self) -> Result<()> {
        let detour_func = Self::hk_mult_sensitivity as *mut c_void as usize;

        let trampoline = unsafe { libmem::hook_code(self.target_address, detour_func) }
            .with_context(|| format!("failed to hook function at {:#x}", self.target_address))?;

        TRAMPOLINE_ADDR.store(trampoline.address, std::sync::atomic::Ordering::SeqCst);
        self.trampoline = Some(trampoline);

        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(self.target_address, trampoline);
            }
        }

        Ok(())
    }
}
