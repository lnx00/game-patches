use windows::Win32::System::Console::{AllocConsole, FreeConsole, SetConsoleTitleW};
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
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

        let dos_header = &*(base_address as *const IMAGE_DOS_HEADER);
        if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        let nt_headers_ptr =
            base_address.offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64;
        let nt_headers = &*nt_headers_ptr;

        if nt_headers.Signature != IMAGE_NT_SIGNATURE {
            return None;
        }

        Some(nt_headers.FileHeader.TimeDateStamp)
    }
}
