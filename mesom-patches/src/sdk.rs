use framework::utils::platform;

pub mod offsets;

const GAME_BINARY_TIMESTAMP: u32 = 0x5FDE56CF;

/// Blocks the caller until the game is fully ready and initialized.
pub fn wait_until_ready() -> Result<(), String> {
    // Wait for game module
    tracing::info!("waiting for game module...");
    let module = offsets::GAME_MODULE.wait();
    tracing::info!("found game module: {}", module);

    // Check game version
    tracing::info!("checking game version...");
    match check_game_version() {
        Ok(version) => tracing::info!("game version ({:X}) validated", version),
        Err(e) => tracing::warn!("failed to check game version: {}", e),
    }

    Ok(())
}

pub fn cleanup() -> Result<(), String> {
    Ok(())
}

pub fn check_game_version() -> Result<u32, String> {
    if let Some(current_timestamp) = platform::get_time_date_stamp() {
        if current_timestamp != GAME_BINARY_TIMESTAMP {
            return Err(format!(
                "timestamp mismatch - expected {}, got {}",
                GAME_BINARY_TIMESTAMP, current_timestamp
            ));
        }

        return Ok(current_timestamp);
    }

    Err("failed to retrieve timestamp".to_string())
}
