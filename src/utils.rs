pub fn patch_bytes(address: usize, bytes: &[u8]) -> Result<(), String> {
    unsafe {
        let old_protect = libmem::prot_memory(address, 0, libmem::Prot::XRW)
            .ok_or("failed to change protection")?;

        libmem::write_memory(address, bytes);

        libmem::prot_memory(address, 0, old_protect).ok_or("failed to restore protection")?;
    }

    Ok(())
}
