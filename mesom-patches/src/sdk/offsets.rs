use framework::{LazyModule, LazySignature};

/* Modules */
pub static GAME_MODULE: LazyModule = LazyModule::new("ShadowOfMordor.exe");

/* Signatures */

// 0x140560EA3: test al, al
pub static LOAD_CAMERA_SMOOTHING_FACTOR: LazySignature =
    LazySignature::new(&GAME_MODULE, "84 C0 75 ? F3 0F 10 25");

// 0x140561007: mulss xmm1, xmm13
pub static MULT_X_AXIS_DELTA_TIME: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 41 0F 59 CD F3 0F 58 C1 F3 0F 11 45");

// 0x140561024: mulss xmm2, xmm13
pub static MULT_Y_AXIS_DELTA_TIME: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 41 0F 59 D5 F3 44 0F 11 5D");

// 0x140560FC8: movss xmm1, cs:const_flt_200
pub static LOAD_X_AXIS_FACTOR: LazySignature = LazySignature::new(
    &GAME_MODULE,
    "F3 0F 10 0D ? ? ? ? F3 0F 10 15 ? ? ? ? EB ? F3 0F 10 54 24",
);
