use std::sync::OnceLock;

use crate::utils;

pub struct LazySignature {
    module: &'static str,
    pattern: &'static str,
    address: OnceLock<Option<usize>>,
}

impl LazySignature {
    pub const fn new(module: &'static str, pattern: &'static str) -> Self {
        Self {
            module,
            pattern,
            address: OnceLock::new(),
        }
    }

    pub fn get(&self) -> Option<usize> {
        *self.address.get_or_init(|| self.scan())
    }

    pub fn scan(&self) -> Option<usize> {
        utils::sig_scan_module(self.module, self.pattern)
    }
}
