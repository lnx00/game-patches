use framework::{LazyModule, LazySignature};

/* Modules */
pub static GAME_MODULE: LazyModule = LazyModule::new("ACS.exe");

/* Signatures */
pub static JUMP_CAMERA_SMOOTHING: LazySignature =
    LazySignature::new(&GAME_MODULE, "75 ? 80 7D ? ? 75 ? 48 8B D9");
