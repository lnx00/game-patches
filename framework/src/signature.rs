use anyhow::{Result, anyhow, bail};
use std::{sync::OnceLock, thread, time::Duration};

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

    pub fn get(&self) -> Result<&libmem::Module> {
        self.module
            .get()
            .or_else(|| {
                let module = libmem::find_module(self.name)?;
                tracing::debug!("found module: {}", module);

                let _ = self.module.set(module);
                self.module.get()
            })
            .ok_or_else(|| anyhow!("module not found"))
    }

    pub fn wait(&self) -> &libmem::Module {
        if let Ok(module) = self.get() {
            return module;
        }

        loop {
            if let Ok(module) = self.get() {
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

    pub fn get(&self) -> Result<usize> {
        if let Some(&addr) = self.address.get() {
            return Ok(addr);
        }

        let module = self.module.get()?;
        let address = Self::scan(module, self.pattern)?;

        let _ = self.address.set(address);
        Ok(address)
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

    fn scan(module: &libmem::Module, signature: &str) -> Result<usize> {
        if let Some(result) = unsafe { libmem::sig_scan(signature, module.base, module.size) } {
            tracing::debug!(
                "found signature: '{}' in '{}' at '{:X}'",
                signature,
                module.name,
                result
            );
            return Ok(result);
        }

        bail!("signature not found: '{}' in '{}'", signature, module.name)
    }
}
