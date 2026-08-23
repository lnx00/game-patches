use framework::{LazyModule, LazySignature};

/* Modules */

// 0x140000000
pub static GAME_MODULE: LazyModule = LazyModule::new("LEGOSTARWARSSKYWALKERSAGA_DX11.exe");

/* Signatures */

// LEGOSTARWARSSKYWALKERSAGA_DX11.exe+149CF0D: movss xmm0, cs:CONST_FLT_0_5
pub static LOAD_DECAY_RATE_ROAMING: LazySignature = LazySignature::new(
    &GAME_MODULE,
    "F3 0F 10 05 ? ? ? ? 0F 2F C7 72 ? 41 0F 28 CA",
);

// LEGOSTARWARSSKYWALKERSAGA_DX11.exe+14924CA: movss xmm0, dword ptr [rcx+0D10h]
pub static LOAD_DECAY_RATE_MOUNTED: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 10 81 ? ? ? ? 41 0F 2E C0 F3 0F 10 B1");

// LEGOSTARWARSSKYWALKERSAGA_DX11.exe+149CF1: jb short loc_14149CF25
pub static SMOOTHING_FALLBACK_COND_ROAMING: LazySignature =
    LazySignature::new(&GAME_MODULE, "72 ? 41 0F 28 CA E8");

// LEGOSTARWARSSKYWALKERSAGA_DX11.exe+149F0AF: mulss xmm0, cs:g_input_factor
pub static MULT_INPUT_FACTOR: LazySignature =
    LazySignature::new(&GAME_MODULE, "45 33 F6 4C 89 B3 ? ? ? ? 48 8B CD");
