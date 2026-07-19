pub mod byte_patch;
pub mod lazy;
pub mod platform;

use std::{
    ffi::c_void,
    sync::LazyLock,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, NTSTATUS},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
            Threading::{GetCurrentProcessId, OpenProcess, PROCESS_VM_OPERATION},
        },
    },
    core::s,
};

type NtProtectVirtualMemoryFn = unsafe extern "system" fn(
    process_handle: HANDLE,
    base_address: *mut *mut c_void,
    region_size: *mut usize,
    new_protect: PAGE_PROTECTION_FLAGS,
    old_protect: *mut PAGE_PROTECTION_FLAGS,
) -> NTSTATUS;

static NT_PROTECT_VIRTUAL_MEMORY: LazyLock<NtProtectVirtualMemoryFn> = LazyLock::new(|| unsafe {
    let ntdll = GetModuleHandleA(s!("ntdll.dll")).unwrap();

    let fn_ptr = GetProcAddress(ntdll, s!("NtProtectVirtualMemory")).unwrap();
    let func: NtProtectVirtualMemoryFn = std::mem::transmute(fn_ptr);

    func
});

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

/// Patches the given bytes.
/// Uses NtProtectVirtualMemory instead of VirtualProtect to bypass some anti-tamper checks.
pub fn patch_bytes_nt(address: usize, bytes: &[u8]) -> Result<()> {
    unsafe {
        // Open handle with proper access privileges
        let process_id = GetCurrentProcessId();
        let process_handle = OpenProcess(PROCESS_VM_OPERATION, false, process_id)
            .expect("failed to open process handle");

        let mut base_address = address as *mut c_void;
        let mut size = bytes.len();
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        // Change protection to RWX
        let status = NT_PROTECT_VIRTUAL_MEMORY(
            process_handle,
            &mut base_address,
            &mut size,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        );

        if status.is_err() {
            let _ = CloseHandle(process_handle);
            bail!("NtProtectVirtualMemory failed with status: {:#X}", status.0)
        }

        // Write the bytes
        libmem::write_memory(address, bytes);

        // Restore previous protection
        let status = NT_PROTECT_VIRTUAL_MEMORY(
            process_handle,
            &mut base_address,
            &mut size,
            old_protect,
            &mut old_protect,
        );

        if status.is_err() {
            let _ = CloseHandle(process_handle);
            bail!("NtProtectVirtualMemory failed with status: {:#X}", status.0)
        }

        let _ = CloseHandle(process_handle);
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
