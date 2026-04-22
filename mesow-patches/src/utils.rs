pub mod platform;

pub unsafe fn patch_bytes(address: usize, bytes: &[u8]) -> Result<(), String> {
    unsafe {
        let old_protect = libmem::prot_memory(address, bytes.len(), libmem::Prot::XRW)
            .ok_or("failed to change protection")?;

        libmem::write_memory(address, bytes);

        libmem::prot_memory(address, bytes.len(), old_protect).ok_or("failed to restore protection")?;

        Ok(())
    }
}
