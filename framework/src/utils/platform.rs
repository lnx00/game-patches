use std::ops::Range;

use windows::Win32::System::Console::{AllocConsole, FreeConsole, SetConsoleTitleW};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_FILE_HEADER, IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{HSTRING, PCWSTR};

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
