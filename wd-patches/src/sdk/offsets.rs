// base: 0x7FFFDCF30000

pub mod sigs {
    // 0x7FFFDE56CA42: call apply_input_acceleration
    pub const CALL_MOUSE_ACCELERATION: &str = "E8 ? ? ? ? 48 8B 5F ? 48 8B 5B ? FF 43";

    // 0x7FFFDE47CA39: jbe loc_7FFFDE47CAD1
    pub const CLAMP_INPUT_CONDITION: &str = "0F 86 ? ? ? ? F3 0F 10 83 ? ? ? ? 48 8B 43";
}