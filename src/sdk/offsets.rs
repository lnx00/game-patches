pub mod sigs {
    // 0x140560EA3: test al, al
    pub const LOAD_CAMERA_SMOOTHING_FACTOR: &str = "84 C0 75 ? F3 0F 10 25";

    // 0x140561007: mulss xmm1, xmm13
    pub const MULT_X_AXIS_DELTA_TIME: &str = "F3 41 0F 59 CD F3 0F 58 C1 F3 0F 11 45";

    // 0x140561024: mulss xmm2, xmm13
    pub const MULT_Y_AXIS_DELTA_TIME: &str = "F3 41 0F 59 D5 F3 44 0F 11 5D";

    // 0x140560FC8: movss xmm1, cs:const_flt_200
    pub const LOAD_X_AXIS_FACTOR: &str = "F3 0F 10 0D ? ? ? ? F3 0F 10 15 ? ? ? ? EB ? F3 0F 10 54 24";
}