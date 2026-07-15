use framework::{LazyModule, LazySignature};

/* Modules */
pub static GAME_MODULE: LazyModule = LazyModule::new("ACS.exe");

/* Signatures */

// 0x14196BEEC: jnz short loc_14196BF68
pub static JUMP_CAMERA_SMOOTHING: LazySignature =
    LazySignature::new(&GAME_MODULE, "75 ? 80 7D ? ? 75 ? 48 8B D9");
