use std::thread;

pub mod integrity;

const CAMERA_MANAGER_ADDRESS: usize = 0x14521AAD0;

#[repr(C)]
pub struct Clock {
    pad_0000: [u8; 0x18], // 0x00
    pub delta_time: f32,  // 0x18
}

#[repr(C)]
pub struct ACUPlayerCameraComponent;

pub fn disable_integrity_checks() -> Result<(), String> {
    integrity::IntegrityHook::inst().apply()?;
    integrity::terminate_integrity_checks()?;

    Ok(())
}

pub fn cleanup_integrity_checks() -> Result<(), String> {
    integrity::IntegrityHook::inst().cleanup()?;

    Ok(())
}

/// Returns whether the ACUPlayerCameraComponent is available.
/// Credits: ACUFixes by NameTaken3125
pub fn is_camera_available() -> bool {
    unsafe {
        // Get the camera manager pointer
        let camera_manager = *(CAMERA_MANAGER_ADDRESS as *const usize);
        if camera_manager == 0 {
            return false;
        }

        // Get the array of camera components
        let array_of_camera_components = camera_manager + 0x40;

        // Get the size of the array
        let array_size = *((array_of_camera_components + 0xA) as *const u16);
        if array_size == 0 {
            return false;
        }

        // Check if the first camera component exists
        let camera_component =
            *(array_of_camera_components as *const *const ACUPlayerCameraComponent);

        !camera_component.is_null()
    }
}

/// Blocks the caller until the game's memory is ready to be patched.
pub fn wait_for_game() {
    while !is_camera_available() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    thread::sleep(std::time::Duration::from_secs(3));
}
