use std::thread;

pub mod integrity;

pub fn disable_integrity_checks() -> Result<(), String> {
    integrity::IntegrityHook::inst().apply()?;
    integrity::terminate_integrity_checks()?;

    Ok(())
}

pub fn cleanup_integrity_checks() -> Result<(), String> {
    integrity::IntegrityHook::inst().cleanup()?;

    Ok(())
}

/// Blocks the caller until the game's memory is ready to be patched.
/// Returns true if ready, false if timeout occurred.
pub fn wait_for_game(timeout_s: u64) -> bool {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_s);

    while !integrity::was_disabled() {
        if start.elapsed() >= timeout {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    thread::sleep(std::time::Duration::from_secs(3));
    return true;
}
