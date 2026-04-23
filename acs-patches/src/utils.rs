#[allow(dead_code)]

use std::{ffi::c_void, sync::LazyLock};

use windows::{
    core::s, Win32::{
        Foundation::{HANDLE, NTSTATUS},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
            Threading::{
                GetCurrentProcessId, OpenProcess, PROCESS_VM_OPERATION,
            },
        },
    }
};

type NtProtectVirtualMemoryFn = unsafe extern "system" fn(
    ProcessHandle: HANDLE,
    BaseAddress: *mut *mut c_void,
    RegionSize: *mut usize,
    NewProtect: PAGE_PROTECTION_FLAGS,
    OldProtect: *mut PAGE_PROTECTION_FLAGS,
) -> NTSTATUS;

static NT_PROTECT_VIRTUAL_MEMORY: LazyLock<NtProtectVirtualMemoryFn> = LazyLock::new(|| unsafe {
    let ntdll = GetModuleHandleA(s!("ntdll.dll")).unwrap();

    let fn_ptr = GetProcAddress(ntdll, s!("NtProtectVirtualMemory")).unwrap();
    let func: NtProtectVirtualMemoryFn = std::mem::transmute(fn_ptr);

    func
});

/// Patches the given bytes.
pub fn patch_bytes(address: usize, bytes: &[u8]) -> Result<(), String> {
    unsafe {
        let old_protect = libmem::prot_memory(address, 0, libmem::Prot::XRW)
            .ok_or("failed to change protection")?;

        libmem::write_memory(address, bytes);

        libmem::prot_memory(address, 0, old_protect).ok_or("failed to restore protection")?;
    }

    Ok(())
}

/// Patches the given bytes.
/// Uses NtProtectVirtualMemory instead of VirtualProtect to bypass some anti-tamper checks.
pub fn patch_bytes_nt(address: usize, bytes: &[u8]) -> Result<(), String> {
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
            return Err(format!(
                "NtProtectVirtualMemory failed with status: {:#X}",
                status.0
            ));
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
            return Err(format!(
                "NtProtectVirtualMemory failed with status: {:#X}",
                status.0
            ));
        }
    }

    Ok(())
}
