use anyhow::Result;
use framework::Patch;
use windows::Win32::System::Threading::{
    GetCurrentProcess, GetProcessAffinityMask, SetProcessAffinityMask,
};

/*
    The game has terrible performance on modern system with a large amount
    of cores. We can fix this by limiting the game to only use 4 cores using
    an affinity mask.

    We try multiple masks to find the best one for the current CPU.
*/

pub struct CpuAffinityFix {
    original_mask: usize,
}

impl Patch for CpuAffinityFix {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "CPU Affinity Fix"
    }

    fn config_key(&self) -> Option<&'static str> {
        Some("cpu_affinity_fix")
    }

    fn init() -> Result<Box<dyn Patch>>
    where
        Self: Sized,
    {
        let mut process_mask: usize = 0x0;
        let mut system_mask: usize = 0x0;

        unsafe {
            let me = GetCurrentProcess();
            GetProcessAffinityMask(me, &mut process_mask, &mut system_mask)?;
        }

        Ok(Box::new(Self {
            original_mask: process_mask,
        }))
    }

    fn apply(&mut self) -> Result<()> {
        const AFFINITY_MASK_01: usize = 0b01010101; // Cores 0, 2, 4, 6
        const AFFINITY_MASK_02: usize = 0b00001111; // Cores 0, 1, 2, 3

        unsafe {
            let me = GetCurrentProcess();
            SetProcessAffinityMask(me, AFFINITY_MASK_01)
                .or_else(|_| SetProcessAffinityMask(me, AFFINITY_MASK_02))?;
        }

        Ok(())
    }

    fn revert(&mut self) -> Result<()> {
        unsafe {
            let me = GetCurrentProcess();
            SetProcessAffinityMask(me, self.original_mask)?;
        }

        Ok(())
    }
}
