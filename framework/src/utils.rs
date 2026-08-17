pub mod byte_patch;
pub mod lazy;
pub mod platform;

use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE;

/// Replaces the given bytes.
pub fn patch_bytes(address: usize, bytes: &[u8]) -> Result<()> {
    unsafe {
        let old_protect = libmem::prot_memory(address, bytes.len(), libmem::Prot::XRW)
            .with_context(|| format!("failed to change protection at {:#x}", address))?;

        libmem::write_memory(address, bytes);

        libmem::prot_memory(address, bytes.len(), old_protect)
            .with_context(|| format!("failed to restore protection at {:#x}", address))?;

        Ok(())
    }
}

/// Replaces the given bytes.
/// Uses VirtualProtectEx on an new unrestricted handle to bypass some anti-tamper checks.
pub fn patch_bytes_ex(address: usize, bytes: &[u8]) -> Result<()> {
    unsafe {
        let me = platform::current_process_ex()?;

        // Unprotect
        let old_protect =
            platform::prot_memory_ex(&me, address, bytes.len(), PAGE_EXECUTE_READWRITE)?;

        // Write
        libmem::write_memory(address, bytes);

        // Restore
        platform::prot_memory_ex(&me, address, bytes.len(), old_protect)?;
    }

    Ok(())
}

pub fn extract_displacement(inst: &libmem::Inst) -> Option<isize> {
    match inst.bytes.as_slice() {
        // call & jmp (rel32)
        [0xE8 | 0xE9, displacement @ ..] if displacement.len() == 4 => {
            Some(i32::from_le_bytes(displacement.try_into().ok()?) as isize)
        }

        // jmp (rel8)
        [0xEB, displacement] => Some(*displacement as i8 as isize),

        // jcc (rel32)
        [0x0F, op, displacement @ ..] if (0x80..=0x8F).contains(op) && displacement.len() == 4 => {
            Some(i32::from_le_bytes(displacement.try_into().ok()?) as isize)
        }

        // mov r64, [rip + disp32] & mov [rip + disp32], r64
        [0x48, 0x89 | 0x8B, modrm, displacement @ ..]
            if (modrm & 0xC7) == 0x05 && displacement.len() == 4 =>
        {
            Some(i32::from_le_bytes(displacement.try_into().ok()?) as isize)
        }

        // movss xmm, [rip + disp32] & movss [rip + disp32], xmm
        [0xF3, 0x0F, 0x10 | 0x11, modrm, displacement @ ..]
            if (modrm & 0xC7) == 0x05 && displacement.len() == 4 =>
        {
            Some(i32::from_le_bytes(displacement.try_into().ok()?) as isize)
        }

        _ => None,
    }
}

/// Resolves the relative target and returns the absolute address
pub fn resolve_relative_target(inst: &libmem::Inst) -> Option<usize> {
    let displacement = extract_displacement(inst)?;
    let next_address = inst.address + inst.bytes.len();

    next_address.checked_add_signed(displacement)
}

/// Returns the rel32 target for jmp
//#[cfg(target_arch = "x86")]
pub fn get_jump_rel32(source_addr: usize, dest_addr: usize) -> i32 {
    (dest_addr as isize - (source_addr as isize + 0x5)) as i32
}

/// Creates jmp istruction and fills the rest with NOPs
pub fn create_jmp_patch_near<const N: usize>(source_addr: usize, dest_addr: usize) -> [u8; N] {
    const { assert!(N >= 5, "buffer too small for jmp patch") };

    let mut patch = [0x90; N];
    patch[0] = 0xE9; // jmp

    // Relative jump target
    let rel32 = get_jump_rel32(source_addr, dest_addr).to_le_bytes();
    patch[1..5].copy_from_slice(&rel32);

    patch
}

// TODO: Merge this into one function
pub fn create_jmp_patch_far<const N: usize>(dest_addr: usize) -> [u8; N] {
    const { assert!(N >= 14, "buffer too small for jmp patch") };

    let mut patch = [0x90; N];
    patch[0..6].copy_from_slice(&[0xFF, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp qword ptr [rip + 0]

    // Relative jump target
    let dest64 = (dest_addr as u64).to_le_bytes();
    patch[6..14].copy_from_slice(&dest64);

    patch
}

/// Verbose version comparison
pub fn check_game_version(expected: &[u32]) -> Result<u32> {
    if let Some(current_timestamp) = platform::get_time_date_stamp() {
        if !expected.contains(&current_timestamp) {
            bail!(
                "timestamp mismatch (got {:#X}, expected one of {:x?})",
                current_timestamp,
                expected
            );
        }

        return Ok(current_timestamp);
    }

    bail!("failed to retrieve timestamp")
}

/// Wait for a boolean to become true
pub fn wait_until_true<F>(timeout: Duration, interval: Duration, mut cond: F) -> Result<()>
where
    F: FnMut() -> bool,
{
    let start = Instant::now();

    while start.elapsed() < timeout {
        if cond() {
            return Ok(());
        }

        thread::sleep(interval);
    }

    bail!("timeout")
}
