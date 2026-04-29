pub mod offsets {
    pub const ROOT_CLOCK_ADDRESS: usize = 0x14525D9D0;
    pub const GET_AXIS_MOVEMENT_ADDRESS: usize = 0x141F6A320;
}

pub mod sigs {
    // 0x141F477D7: jz short loc_141F477EF
    pub const JUMP_CAMERA_SMOOTHING: &str = "74 ? 41 8B 06 41 89 85";
}