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
    address: OnceLock<Option<usize>>,
}

impl LazySignature {
    pub const fn new(module: &'static LazyModule, pattern: &'static str) -> Self {
        Self {
            module,
            pattern,
            address: OnceLock::new(),
        }
    }

    pub fn get(&self) -> Option<usize> {
        if let Some(address) = self.address.get() {
            return *address;
        }

        let module = self.module.get()?;

        return *self
            .address
            .get_or_init(|| Self::scan(module, self.pattern));
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
