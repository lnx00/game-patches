use std::{
    ffi::{CString, c_char},
    io,
    sync::OnceLock,
};

use assert_offset::AssertOffsets;
use tracing_subscriber::fmt::writer::MakeWriter;
use windows::Win32::Foundation::HMODULE;

pub struct ImGuiShared;
pub struct InputHooks;
pub struct AnimationModdingInterface;

pub const fn make_version(major: u64, minor: u64, minorer: u64, minorest: u64) -> u64 {
    (major << 24) | (minor << 16) | (minorer << 8) | minorest
}

pub const PLUGIN_API_VERSION: u64 = make_version(0, 9, 1, 0);

static IMGUI_CONSOLE: OnceLock<&'static ImGuiConsoleInterface> = OnceLock::new();

#[derive(AssertOffsets)]
#[repr(C)]
pub struct ImGuiConsoleInterface {
    #[offset(0x0)]
    pub fnp_add_log: Option<unsafe extern "C" fn(s: *const c_char)>,
}

impl ImGuiConsoleInterface {
    pub fn add_log(&self, text: &str) {
        if let Some(log_fn) = self.fnp_add_log {
            if let Ok(cstr) = CString::new(text) {
                unsafe {
                    log_fn(cstr.as_ptr());
                }
            }
        }
    }
}

struct ImGuiMakeWriter;

impl<'a> MakeWriter<'a> for ImGuiMakeWriter {
    type Writer = ImGuiWriter;

    fn make_writer(&self) -> Self::Writer {
        ImGuiWriter {
            console: IMGUI_CONSOLE.get().expect("ImGui console not initialized"),
        }
    }
}

struct ImGuiWriter {
    console: &'static ImGuiConsoleInterface,
}

impl io::Write for ImGuiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                println!("{trimmed}");
                self.console.add_log(trimmed);
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(AssertOffsets)]
#[repr(C)]
pub struct ACUPluginLoaderSharedGlobals {
    #[offset(0x0)]
    pub input_hooks: *mut InputHooks,

    #[offset(0x8)]
    pub animation_modding: *mut AnimationModdingInterface,

    #[offset(0x10)]
    pub imgui_console: *mut ImGuiConsoleInterface,
}

#[derive(AssertOffsets)]
#[repr(C)]
pub struct ACUPluginLoaderInterface {
    #[offset(0x0)]
    pub m_plugin_loader_version: u64,

    #[offset(0x8)]
    pub request_unload_plugin: Option<unsafe extern "C" fn(dll_handle: HMODULE)>,

    #[offset(0x10)]
    pub get_plugin_if_loaded: Option<unsafe extern "C" fn(plugin_name: *const u16) -> HMODULE>,

    #[offset(0x18)]
    pub m_implementation_shared_variables: *mut ACUPluginLoaderSharedGlobals,
}

impl ACUPluginLoaderInterface {
    pub fn init_logger(&self) {
        if let Some(console) = unsafe {
            self.m_implementation_shared_variables
                .as_ref()
                .and_then(|globals| globals.imgui_console.as_ref())
        } {
            let _ = IMGUI_CONSOLE.set(console);
        }

        let subscriber = tracing_subscriber::fmt()
            .with_writer(ImGuiMakeWriter)
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    }
}

#[derive(AssertOffsets)]
#[repr(C)]
pub struct ACUPluginInfo {
    #[offset(0x0)]
    pub m_plugin_api_version: u64,

    #[offset(0x8)]
    pub m_plugin_version: u64,

    #[offset(0x10)]
    pub m_init_stage_when_code_patches_are_safe_to_apply:
        Option<extern "C" fn(plugin_loader: &ACUPluginLoaderInterface) -> bool>,

    #[offset(0x18)]
    pub m_every_frame_when_menu_is_open: Option<extern "C" fn(imgui_context: &ImGuiShared)>,

    #[offset(0x20)]
    pub m_every_frame_even_when_menu_is_closed: Option<extern "C" fn(imgui_context: &ImGuiShared)>,

    #[offset(0x28)]
    pub m_init_stage_when_versions_are_deemed_compatible:
        Option<extern "C" fn(plugin_loader: &ACUPluginLoaderInterface)>,

    #[offset(0x30)]
    pub m_early_hook_when_game_code_is_unpacked: Option<extern "C" fn()>,
}
