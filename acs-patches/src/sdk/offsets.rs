pub mod sigs {
    use framework::LazySignature;
    use crate::sdk;

    // 0x14196BEEC: jnz short loc_14196BF68
    pub const JUMP_CAMERA_SMOOTHING: LazySignature =
        LazySignature::new(sdk::GAME_MODULE_NAME, "75 ? 80 7D ? ? 75 ? 48 8B D9");
}
