use framework::{LazyModule, LazySignature};

/* Modules */

// 0x140000000
pub static GAME_MODULE: LazyModule = LazyModule::new("ShadowOfWar.exe");

/* Signatures */

// ShadowOfWar.exe+516BEB: movss xmm3, dword ptr [rax]
pub static LOAD_CAMERA_SMOOTHING_FACTORS: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 10 18 F3 0F 10 78 ? F3 0F 10 55");
