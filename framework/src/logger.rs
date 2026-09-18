use std::{
    io::{self, Write},
    sync::{LazyLock, OnceLock},
    time::Instant,
};

use anyhow::{Context, Result};
use log::{Level, LevelFilter, SetLoggerError};

use crate::utils::{self, platform};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_DIM: &str = "\x1b[2m";

pub struct Logger {
    pub max_level: LevelFilter,
    pub ansi_support: bool,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();
static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

impl Logger {
    pub fn init(config: Logger) -> Result<(), SetLoggerError> {
        let max_level = config.max_level;

        if LOGGER.set(config).is_ok() {
            log::set_logger(LOGGER.get().unwrap())?;
            log::set_max_level(max_level);
        }

        Ok(())
    }

    fn elapsed_timestamp() -> String {
        let elapsed = START_TIME.elapsed();
        format!("{:>4}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let (level_color, level_str) = match record.level() {
            Level::Error => ("\x1b[31m", "ERROR"),
            Level::Warn => ("\x1b[33m", " WARN"),
            Level::Info => ("\x1b[32m", " INFO"),
            Level::Debug => ("\x1b[34m", "DEBUG"),
            Level::Trace => ("\x1b[35m", "TRACE"),
        };

        let timestamp = Self::elapsed_timestamp();

        let mut stderr = io::stderr().lock();

        if self.ansi_support {
            let _ = writeln!(
                stderr,
                "{ANSI_DIM}{timestamp}{ANSI_RESET} {level_color}{level_str}{ANSI_RESET} {ANSI_DIM}{}:{ANSI_RESET} {}",
                record.target(),
                record.args()
            );
        } else {
            let _ = writeln!(
                stderr,
                "{timestamp}{level_str} {}: {}",
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        let _ = io::stderr().flush();
    }
}

/// Initializes the logger. Silently fails, if it was already initialized.
pub fn init_logger(max_level: log::LevelFilter) {
    let config = Logger {
        max_level,
        ansi_support: true,
    };

    let _ = Logger::init(config);
}
