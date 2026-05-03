use std::{
    collections::HashMap,
    convert::TryFrom,
    ffi::c_void,
    ops::Range,
    sync::{
        LazyLock, Mutex, MutexGuard, OnceLock, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use windows::{
    Wdk::System::Threading::{NtQueryInformationThread, ThreadQuerySetWin32StartAddress},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::{
            Diagnostics::Debug::{IMAGE_FILE_HEADER, IMAGE_SECTION_HEADER},
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE},
            Threading::{OpenThread, THREAD_ALL_ACCESS, TerminateThread},
        },
    },
    core::{PCSTR, s},
};

use crate::utils::WaitLock;

#[cfg(target_pointer_width = "64")]
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64 as IMAGE_NT_HEADERS;

#[cfg(target_pointer_width = "32")]
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32 as IMAGE_NT_HEADERS;

type CreateThreadFn = unsafe extern "system" fn(
    lp_thread_attributes: *mut c_void,
    dw_stack_size: usize,
    lp_start_address: *mut c_void,
    lp_parameter: *mut c_void,
    dw_creation_flags: u32,
    lp_thread_id: *mut u32,
) -> HANDLE;

static ORIG_CREATE_THREAD: OnceLock<CreateThreadFn> = OnceLock::new();

pub fn find_section_address_range(section_name: &str) -> Option<Range<usize>> {
    let mut target_name_bytes = [0u8; 8];
    let bytes = section_name.as_bytes();
    let len = bytes.len().min(8);
    target_name_bytes[..len].copy_from_slice(&bytes[..len]);

    unsafe {
        // Base address
        let h_module = GetModuleHandleA(PCSTR::null()).ok()?;
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
        let nt_headers_ptr = base_addr.add(dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS;
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

static INTEGRITY_SECTION_RANGE: LazyLock<Option<Range<usize>>> =
    LazyLock::new(|| find_section_address_range(".UBX0"));

static INTEGRITY_THREAD_VERDICTS: LazyLock<RwLock<HashMap<usize, bool>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static INTEGRITY_THREAD_FOUND: AtomicBool = AtomicBool::new(false);

/// Extract the relative target address (jmp or call)
fn extract_relative_target(inst: &libmem::Inst) -> Option<usize> {
    let next_address = inst.address as i64 + inst.bytes.len() as i64;

    let target = match inst.bytes.as_slice() {
        [0xE8 | 0xE9, displacement @ ..] if displacement.len() == 4 => {
            let displacement = i32::from_le_bytes(displacement.try_into().ok()?) as i64;
            next_address.checked_add(displacement)?
        }
        [0xEB, displacement] => {
            let displacement = i8::from_le_bytes([*displacement]) as i64;
            next_address.checked_add(displacement)?
        }
        _ => return None,
    };

    usize::try_from(target).ok()
}

/// Analyzes the thread start code and checks if it is the integrity thread
fn analyze_thread_start(start_address: usize) -> Option<bool> {
    unsafe {
        tracing::debug!("analyzing thread {:X}...", start_address);

        // Try to get cached verdict with read lock
        if let Some(verdict) = INTEGRITY_THREAD_VERDICTS
            .read()
            .unwrap()
            .get(&start_address)
            .copied()
        {
            tracing::debug!("cached verdict for thread {:X}: {}", start_address, verdict);
            return Some(verdict);
        }

        if let Some(section_range) = INTEGRITY_SECTION_RANGE.as_ref() {
            let inst = libmem::disassemble(start_address)?;

            let mnemonic = inst.mnemonic.to_lowercase();
            if mnemonic != "jmp" && mnemonic != "call" {
                tracing::debug!("first inst was not a jump or call");
                INTEGRITY_THREAD_VERDICTS
                    .write()
                    .unwrap()
                    .insert(start_address, false);
                return Some(false);
            }

            let target_addr = extract_relative_target(&inst)?;
            tracing::debug!("{} target addr: {}", mnemonic, target_addr);

            let in_range = section_range.contains(&target_addr);

            INTEGRITY_THREAD_VERDICTS
                .write()
                .unwrap()
                .insert(start_address, in_range);

            tracing::debug!("verdict for thread {:X}: {}", start_address, in_range);

            return Some(in_range);
        }

        None
    }
}

fn check_thread(thread_id: u32) -> Result<bool, String> {
    unsafe {
        let mut thread_start_address = 0x0;

        // Get a handle to the thread
        let thread_handle = OpenThread(THREAD_ALL_ACCESS, false, thread_id)
            .map_err(|_| "failed to open thread handle")?;

        // Query the thread start address
        let nt_status = NtQueryInformationThread(
            thread_handle,
            ThreadQuerySetWin32StartAddress,
            &mut thread_start_address as *mut usize as *mut _,
            0x8,
            std::ptr::null_mut(),
        );

        if nt_status.is_err() {
            let _ = CloseHandle(thread_handle);
            return Err(format!(
                "failed to query thread information: {:?}",
                nt_status
            ));
        }

        let is_integrity_thread = analyze_thread_start(thread_start_address) == Some(true);
        if is_integrity_thread {
            let _ = TerminateThread(thread_handle, 0x0);
            tracing::debug!("terminated integrity check thread: {:X}", thread_id);
        }

        let _ = CloseHandle(thread_handle);
        Ok(is_integrity_thread)
    }
}

/// Searches for the integrity check thread and tries to terminate it.
/// Returns Ok(true), if at least one thread has been successfully terminated.
pub fn terminate_integrity_checks() -> Result<bool, String> {
    let process_id = libmem::get_process()
        .ok_or("failed to get current process")?
        .pid;
    let thread_list = libmem::enum_threads().ok_or("failed to enumerate threads")?;
    let mut terminated_any = false;

    // Check all thread of the current process
    for thread in thread_list {
        if thread.owner_pid == process_id {
            let check_result = check_thread(thread.tid);
            match check_result {
                Ok(true) => {
                    tracing::info!("terminated integrity check thread: {:X}", thread.tid);
                    terminated_any = true;
                }

                Err(e) => {
                    tracing::warn!("cannot check thread {:X}: {}", thread.tid, e);
                }

                _ => {}
            }
        }
    }

    Ok(terminated_any)
}

pub fn initialize(timeout: Duration) -> Result<(), String> {
    INTEGRITY_THREAD_FOUND.store(false, Ordering::SeqCst);

    // Install hook
    tracing::info!("installing CreateThread hook...");
    IntegrityHook::inst().apply()?;

    // Terminate running threads
    tracing::info!("terminating existing integrity checks...");
    if terminate_integrity_checks()? {
        return Ok(());
    }

    // Wait until the thread was killed...
    tracing::info!("waiting for new integrity check thread...");
    let start = std::time::Instant::now();
    while !INTEGRITY_THREAD_FOUND.load(Ordering::SeqCst) {
        if start.elapsed() >= timeout {
            return Err("timeout while waiting for integrity check".to_string());
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

pub struct IntegrityHook {
    trampoline: Option<libmem::Trampoline>,
    target_address: usize,
}

static INSTANCE: LazyLock<Mutex<IntegrityHook>> = LazyLock::new(|| {
    // Get the address of kernel32::CreateThread
    let kernel32_handle = unsafe { GetModuleHandleA(s!("kernel32.dll")).unwrap() };
    let fp_create_thread = unsafe { GetProcAddress(kernel32_handle, s!("CreateThread")).unwrap() };

    // Hook the CreateThread function
    let create_thread_address = fp_create_thread as *mut c_void as usize;

    Mutex::new(IntegrityHook {
        trampoline: None,
        target_address: create_thread_address,
    })
});

impl IntegrityHook {
    pub fn inst() -> MutexGuard<'static, IntegrityHook> {
        INSTANCE.lock().unwrap()
    }

    pub fn apply(&mut self) -> Result<(), String> {
        if self.trampoline.is_some() {
            return Ok(());
        }

        unsafe {
            let hook_address = Self::hk_create_thread as *mut c_void as usize;

            let trampoline = libmem::hook_code(self.target_address, hook_address)
                .ok_or("failed to hook CreateThread")?;

            let _ = ORIG_CREATE_THREAD.set(trampoline.callable::<CreateThreadFn>());
            self.trampoline = Some(trampoline);
        }

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), String> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(self.target_address, trampoline);
            }
        }

        Ok(())
    }

    extern "system" fn empty_thread(_: *mut c_void) -> u32 {
        return 0;
    }

    extern "system" fn hk_create_thread(
        lp_thread_attributes: *mut c_void,
        dw_stack_size: usize,
        lp_start_address: *mut c_void,
        lp_parameter: *mut c_void,
        dw_creation_flags: u32,
        lp_thread_id: *mut u32,
    ) -> HANDLE {
        let mut lp_start_address = lp_start_address;

        if analyze_thread_start(lp_start_address as usize) == Some(true) {
            INTEGRITY_THREAD_FOUND.store(true, Ordering::SeqCst);
            lp_start_address = Self::empty_thread as *mut c_void;
            tracing::info!("CreateThread: prevented integrity check thread creation");
        }

        let original = WaitLock::wait(&ORIG_CREATE_THREAD);

        return unsafe {
            original(
                lp_thread_attributes,
                dw_stack_size,
                lp_start_address,
                lp_parameter,
                dw_creation_flags,
                lp_thread_id,
            )
        };
    }
}
