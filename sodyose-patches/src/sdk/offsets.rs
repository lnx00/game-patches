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

// StateOfDecay.exe+11EA5B6: mulss xmm1, [esp+60h+delta_time]
pub static MULT_DELTA_TIME_ROAMING: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 59 4C 24 ? F3 0F 10 87 ? ? ? ? F3 0F 59 54 24");

// StateOfDecay.exe+11E9968: mulss xmm0, [esp+40h+delta_time]
pub static MULT_DELTA_TIME_AIMING_Y: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 59 44 24 ? F3 0F 59 C3 0F 57 05");

// StateOfDecay.exe+11E9995: mulss xmm0, [esp+40h+delta_time]
pub static MULT_DELTA_TIME_AIMING_X: LazySignature =
    LazySignature::new(&GAME_MODULE, "F3 0F 59 44 24 ? F3 A5");
