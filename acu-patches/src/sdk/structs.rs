pub mod structs {

    #[repr(C)]
    pub struct Clock {
        pad_0000: [u8; 0x18], // 0x00
        pub delta_time: f32,  // 0x18
    }
}
