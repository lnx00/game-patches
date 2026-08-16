mod config;
mod patches;
mod sdk;

#[cfg(feature = "plugin")]
#[path = "lib_plugin.rs"]
mod root_impl;

#[cfg(not(feature = "plugin"))]
#[path = "lib_standalone.rs"]
mod root_impl;

pub use root_impl::*;
