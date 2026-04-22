use crate::utils;

pub struct BytePatch<const N: usize> {
    address: usize,
    patch_bytes: [u8; N],
    original_bytes: [u8; N],
    is_applied: bool,
}

impl<const N: usize> BytePatch<N> {
    pub fn new(address: usize, patch_bytes: [u8; N]) -> Self {
        Self {
            address,
            patch_bytes,
            original_bytes: [0; N],
            is_applied: false,
        }
    }

    pub fn apply(&mut self) -> Result<(), String> {
        if self.is_applied {
            return Ok(());
        }

        unsafe {
            self.original_bytes = libmem::read_memory::<_>(self.address);
            utils::patch_bytes(self.address, &self.patch_bytes)?;
        }

        self.is_applied = true;
        Ok(())
    }

    pub fn revert(&mut self) -> Result<(), String> {
        if !self.is_applied {
            return Ok(());
        }

        unsafe {
            utils::patch_bytes(self.address, &self.original_bytes)?;
        }

        self.is_applied = false;
        Ok(())
    }
}
