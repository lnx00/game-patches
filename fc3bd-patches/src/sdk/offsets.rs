use framework::{LazyModule, LazySignature};

/* Modules */

// 0x50EE0000
pub static GAME_MODULE: LazyModule = LazyModule::new("FC3_d3d11.dll");

/* Signatures */

// FC3_d3d11.dll+1A04C17: jbe short loc_528E4C3E
pub static CLAMP_INPUT_CONDITION: LazySignature =
    LazySignature::new(&GAME_MODULE, "76 ? 0F 2F C3 72");

// FC3_d3d11.dll+484DA8: movss xmm0, dword ptr [ecx+40h]
pub static LOAD_SENSITIVITY: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 10 41 ? 8B 41 ? F3 0F 10 0D");
