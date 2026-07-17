use framework::{LazyModule, LazySignature};

/* Modules */

// 0x140000000
pub static GAME_MODULE: LazyModule = LazyModule::new("ACS.exe");

/* Signatures */

// ACS.exe+196BEEC: jnz short loc_14196BF68
pub static JUMP_CAMERA_SMOOTHING: LazySignature =
    LazySignature::new(&GAME_MODULE, "75 ? 80 7D ? ? 75 ? 48 8B D9");
