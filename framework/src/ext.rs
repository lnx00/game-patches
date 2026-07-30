use std::fmt::Display;

pub trait ResultLogExt<T> {
    fn warn_and_continue(self, msg: &str) -> Option<T>;
}

impl<T, E> ResultLogExt<T> for Result<T, E>
where
    E: Display,
{
    fn warn_and_continue(self, msg: &str) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                tracing::warn!("Suppressed error: {}: {:#}", msg, e);
                None
            }
        }
    }
}
