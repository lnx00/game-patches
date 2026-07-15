use framework::{LazyModule, LazySignature};

/* Modules */
pub static GAME_MODULE: LazyModule = LazyModule::new("ACU.exe");

// 0x140032B74: mov rax, cs:g_RootClock
pub static ROOT_CLOCK_ACCESS: LazySignature =
    LazySignature::new(&GAME_MODULE, "48 8B 05 ? ? ? ? 44 0F B7 75");

// 0x141F664BD: call get_axis_movement
pub static GET_AXIS_MOVEMENT_CALL: LazySignature =
    LazySignature::new(&GAME_MODULE, "E8 ? ? ? ? 48 8B 5F ? F3 0F 59 3D");

// 0x141F477D7: jz short loc_141F477EF
pub static JUMP_CAMERA_SMOOTHING: LazySignature =
    LazySignature::new(&GAME_MODULE, "74 ? 41 8B 06 41 89 85");
