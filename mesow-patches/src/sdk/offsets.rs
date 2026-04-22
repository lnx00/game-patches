pub mod sigs {
    // 0x140516BEB: movss xmm3, dword ptr [rax]
    pub const LOAD_CAMERA_SMOOTHING_FACTORS: &str = "F3 0F 10 18 F3 0F 10 78 ? F3 0F 10 55";
}