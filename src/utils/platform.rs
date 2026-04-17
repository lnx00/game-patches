use windows::Win32::System::Console::{AllocConsole, FreeConsole, SetConsoleTitleA};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;

#[allow(dead_code)]
pub enum MsgBoxType {
    Info,
    Warning,
    Error,
}

fn to_pcstr(text: &str) -> PCSTR {
    PCSTR(format!("{}\0", text).as_ptr())
}

pub fn msg_box(msg: &str, title: &str, box_type: MsgBoxType) {
    let icon = match box_type {
        MsgBoxType::Info => MB_ICONINFORMATION,
        MsgBoxType::Warning => MB_ICONWARNING,
        MsgBoxType::Error => MB_ICONERROR,
    };

    unsafe {
        MessageBoxA(None, to_pcstr(msg), to_pcstr(title), MB_OK | icon);
    }
}

pub fn attach_console(title: &str) {
    unsafe {
        let _ = AllocConsole();
        let _ = SetConsoleTitleA(to_pcstr(title));
    }
}

pub fn detach_console() {
    let _ = unsafe { FreeConsole() };
}

pub fn is_button_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}
