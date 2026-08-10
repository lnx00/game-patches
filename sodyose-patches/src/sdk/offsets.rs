use framework::{LazyModule, LazySignature};

/* Modules */

// 0x400000
pub static GAME_MODULE: LazyModule = LazyModule::new("StateOfDecay.exe");

/* Signatures */

// These sigs are garbage...

// StateOfDecay.exe+1188F64: movss dword ptr [ecx+0C8h], xmm0
pub static LIMIT_X_FACTOR_ANCHOR: LazySignature =
    LazySignature::new(&GAME_MODULE, "EB ? F3 0F 58 C1 F3 0F 11 81") /* + 0x41 */;

// StateOfDecay.exe+1188EC4: movss dword ptr [ecx+0CCh], xmm0
pub static LIMIT_Y_FACTOR_ANCHOR: LazySignature =
    LazySignature::new(&GAME_MODULE, "EB ? F3 0F 5C C1 F3 0F 11 81") /* + 0x41 */;

// StateOfDecay.exe+11EA596: jz loc_15EA6DF
pub static INPUT_DEADZONE_COND: LazySignature =
    LazySignature::new(&GAME_MODULE, "0F 84 ? ? ? ? F3 0F 10 4E ? F3 0F 59 8F");
