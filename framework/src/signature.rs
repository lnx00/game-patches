use std::{sync::OnceLock, thread, time::Duration};

use crate::utils;

pub struct LazyModule {
    name: &'static str,
    module: OnceLock<libmem::Module>,
}

impl LazyModule {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            module: OnceLock::new(),
        }
    }

    pub fn get(&self) -> Option<&libmem::Module> {
        self.module.get().or_else(|| {
            let module = libmem::find_module(self.name)?;
            tracing::debug!(
                "found module '{}': {:x} - {:x} ({:x})",
                module.name,
                module.base,
                module.end,
                module.size
            );

            let _ = self.module.set(module);
            self.module.get()
        })
    }

    pub fn wait(&self) -> &libmem::Module {
        if let Some(module) = self.get() {
            return module;
        }

        loop {
            if let Some(module) = self.get() {
                return module;
            }

            thread::sleep(Duration::from_millis(50));
        }
    }
}

pub struct LazySignature {
    module: &'static LazyModule,
    pattern: &'static str,
    address: OnceLock<usize>,
}

impl LazySignature {
    pub const fn new(module: &'static LazyModule, pattern: &'static str) -> Self {
        Self {
            module,
            pattern,
            address: OnceLock::new(),
        }
    }

    pub fn get(&self) -> Result<usize, ()> {
        self.address
            .get()
            .or_else(|| {
                let module = self.module.get()?;
                let address = Self::scan(module, self.pattern)?;

                let _ = self.address.set(address);
                self.address.get()
            })
            .ok_or(())
            .cloned()
    }

    pub fn wait(&self) -> usize {
        if let Ok(address) = self.get() {
            return address;
        }

        loop {
            if let Ok(address) = self.get() {
                return address;
            }

            thread::sleep(Duration::from_millis(50));
        }
    }

    fn scan(module: &libmem::Module, signature: &str) -> Option<usize> {
        if let Some(result) = unsafe { libmem::sig_scan(signature, module.base, module.size) } {
            tracing::debug!(
                "found signature: '{}' in '{}' at '{:X}'",
                signature,
                module.name,
                result
            );
            return Some(result);
        }

        tracing::debug!("signature not found: '{}' in '{}'", signature, module.name);
        None
    }
}
