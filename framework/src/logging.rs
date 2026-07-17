use std::{path::Path, sync::OnceLock};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

pub fn init_logger(log_file: impl AsRef<Path>, log_level: &str) {
    let log_file = log_file.as_ref();
    let directory = log_file.parent().unwrap_or(Path::new("."));
    let file_name = log_file.file_name().unwrap_or_default();

    let file_appender = tracing_appender::rolling::never(directory, file_name);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let _ = LOG_GUARD.set(_guard);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().pretty())
        .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
        .init();
}
