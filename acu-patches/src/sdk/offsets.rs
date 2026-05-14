pub mod sigs {
    // 0x140032B74: mov rax, cs:g_RootClock
    pub const ROOT_CLOCK_ACCESS: &str = "48 8B 05 ? ? ? ? 44 0F B7 75";

    // 0x141F664BD: call get_axis_movement
    pub const GET_AXIS_MOVEMENT_CALL: &str = "E8 ? ? ? ? 48 8B 5F ? F3 0F 59 3D";

    // 0x141F477D7: jz short loc_141F477EF
    pub const JUMP_CAMERA_SMOOTHING: &str = "74 ? 41 8B 06 41 89 85";
}
