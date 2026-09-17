use std::ops::Deref;

use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub struct AutoHandle(pub HANDLE);

impl Deref for AutoHandle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for AutoHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}
