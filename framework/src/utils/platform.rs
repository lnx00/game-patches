use anyhow::{Context, Result, bail};
use std::ffi::c_void;
use std::ops::Range;
use std::sync::LazyLock;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect};

use windows::Win32::System::Console::{AllocConsole, FreeConsole, SetConsoleTitleW};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_FILE_HEADER, IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
};
use windows::Win32::System::Threading::{GetCurrentProcessId, OpenProcess, PROCESS_VM_OPERATION};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{HSTRING, PCWSTR};
use windows::{
    Win32::{
        Foundation::{HANDLE, NTSTATUS},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Memory::PAGE_PROTECTION_FLAGS,
        },
    },
    core::s,
};

pub use enable_ansi_support::enable_ansi_support;

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

#[allow(dead_code)]
pub enum MsgBoxType {
    Info,
    Warning,
    Error,
}

pub fn msg_box(msg: &str, title: &str, box_type: MsgBoxType) {
    let icon = match box_type {
        MsgBoxType::Info => MB_ICONINFORMATION,
        MsgBoxType::Warning => MB_ICONWARNING,
        MsgBoxType::Error => MB_ICONERROR,
    };

    let msg_w = HSTRING::from(msg);
    let title_w = HSTRING::from(title);

    unsafe {
        MessageBoxW(
            None,
            PCWSTR(msg_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | icon,
        );
    }
}

pub fn attach_console(title: &str) {
    let title_w = HSTRING::from(title);

    unsafe {
        let _ = AllocConsole();
        let _ = SetConsoleTitleW(PCWSTR(title_w.as_ptr()));
    }
}

pub fn detach_console() {
    let _ = unsafe { FreeConsole() };
}

pub fn is_button_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}

pub fn get_time_date_stamp() -> Option<u32> {
    unsafe {
        let module_handle = GetModuleHandleW(PCWSTR::null()).ok()?;

        let base_address = module_handle.0 as *const u8;
        if base_address.is_null() {
            return None;
        }

        let dos_header_ptr = base_address as *const IMAGE_DOS_HEADER;
        let dos_header = std::ptr::read_unaligned(dos_header_ptr);

        if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        let nt_headers_ptr =
            base_address.offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64;
        let nt_headers = std::ptr::read_unaligned(nt_headers_ptr);

        if nt_headers.Signature != IMAGE_NT_SIGNATURE {
            return None;
        }

        Some(nt_headers.FileHeader.TimeDateStamp)
    }
}

pub fn find_section_address_range(section_name: &str) -> Option<Range<usize>> {
    let mut target_name_bytes = [0u8; 8];
    let bytes = section_name.as_bytes();
    let len = bytes.len().min(8);
    target_name_bytes[..len].copy_from_slice(&bytes[..len]);

    unsafe {
        // Base address
        let h_module = GetModuleHandleW(PCWSTR::null()).ok()?;
        if h_module.is_invalid() {
            return None;
        }
        let base_addr = h_module.0 as *const u8;

        // DOS Header
        let dos_header = &*(base_addr as *const IMAGE_DOS_HEADER);
        if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        // NT Headers
        let nt_headers_ptr =
            base_addr.add(dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        let nt_headers = &*nt_headers_ptr;

        if nt_headers.Signature != IMAGE_NT_SIGNATURE {
            return None;
        }

        // Section headers start
        let optional_header_offset = 4 + std::mem::size_of::<IMAGE_FILE_HEADER>();
        let section_headers_ptr = (nt_headers_ptr as *const u8)
            .add(optional_header_offset)
            .add(nt_headers.FileHeader.SizeOfOptionalHeader as usize)
            as *const IMAGE_SECTION_HEADER;

        // Convert section headers
        let num_sections = nt_headers.FileHeader.NumberOfSections as usize;
        let sections = std::slice::from_raw_parts(section_headers_ptr, num_sections);

        // Find section
        for section in sections {
            if section.Name == target_name_bytes {
                let virtual_size = section.Misc.VirtualSize as usize;

                let start_address = (base_addr as usize) + section.VirtualAddress as usize;
                let end_address = start_address + virtual_size;

                return Some(start_address..end_address);
            }
        }
    }

    None
}

/// Changes the memory protection using native functions and returns the old protection.
pub fn prot_memory_native(
    address: usize,
    size: usize,
    prot: PAGE_PROTECTION_FLAGS,
) -> Result<PAGE_PROTECTION_FLAGS> {
    unsafe {
        let process_id = GetCurrentProcessId();
        let process_handle = OpenProcess(PROCESS_VM_OPERATION, false, process_id)?;

        let mut base_address = address as *mut c_void;
        let mut region_size = size;
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        let status = NT_PROTECT_VIRTUAL_MEMORY(
            process_handle,
            &mut base_address,
            &mut region_size,
            prot,
            &mut old_protect,
        );
        let _ = CloseHandle(process_handle);

        if status.is_err() {
            bail!("NtProtectVirtualMemory failed with status: {:#X}", status.0)
        }

        Ok(old_protect)
    }
}

/// Unhooks NtProtectVirtualMemory.
/// Credits: https://github.com/yubie-re/vmp-virtualprotect-bypass
#[cfg(target_arch = "x86_64")]
pub fn unhook_prot_memory() -> Result<()> {
    unsafe {
        let ntdll = GetModuleHandleA(s!("ntdll.dll"))?;
        let nt_query_section_addr =
            GetProcAddress(ntdll, s!("NtQuerySection")).context("failed to find NtQuerySection")?;
        let nt_vp_addr = GetProcAddress(ntdll, s!("NtProtectVirtualMemory"))
            .context("failed to find NtProtectVirtualMemory")?;

        // NtProtectVirtualMemory = NtQuerySection - 1
        let syscall_id = (nt_query_section_addr as *const u8)
            .add(4)
            .read()
            .wrapping_sub(1);
        tracing::debug!("NtProtectVirtualMemory syscall id: {:#x}", syscall_id);

        // ntdll syscall stub layout:
        //   0: 4C 8B D1        mov r10, rcx
        //   3: B8 XX 00 00 00  mov eax, <syscall_number>
        let restore: [u8; 5] = [0x4C, 0x8B, 0xD1, 0xB8, syscall_id];
        let target = nt_vp_addr as *mut u8;

        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        VirtualProtect(
            target.cast(),
            restore.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )?;

        std::ptr::copy_nonoverlapping(restore.as_ptr(), target, restore.len());

        VirtualProtect(target.cast(), restore.len(), old_protect, &mut old_protect)?;

        Ok(())
    }
}
