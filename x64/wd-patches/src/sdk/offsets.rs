use framework::{LazyModule, LazySignature};

/* Modules */

// 0x7FFFDCF30000
pub static GAME_MODULE: LazyModule = LazyModule::new("Disrupt_b64.dll");

/* Signatures */

// Disrupt_b64.dll+163CA42: call apply_input_acceleration
pub static CALL_MOUSE_ACCELERATION: LazySignature =
    LazySignature::new(&GAME_MODULE, "E8 ? ? ? ? 48 8B 5F ? 48 8B 5B ? FF 43");

// Disrupt_b64.dll+154CA39: jbe loc_7FFFDE47CAD1
pub static CLAMP_INPUT_CONDITION: LazySignature =
    LazySignature::new(&GAME_MODULE, "0F 86 ? ? ? ? F3 0F 10 83 ? ? ? ? 48 8B 43");

// Disrupt_b64.dll+179BE2A: jz short loc_7FFFDE6CBE89
pub static APPLY_DRIVING_DEADZONE_COND: LazySignature =
    LazySignature::new(&GAME_MODULE, "74 ? 44 0F 2F 0D ? ? ? ? 0F 28 D6");
