use std::{
    convert::TryFrom,
    ffi::c_void,
    ops::Range,
    sync::{LazyLock, Mutex, MutexGuard},
};

use windows::{
    Wdk::System::Threading::{NtQueryInformationThread, ThreadQuerySetWin32StartAddress},
    Win32::{
        Foundation::HANDLE,
        System::{
            Diagnostics::Debug::{IMAGE_FILE_HEADER, IMAGE_SECTION_HEADER},
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE},
            Threading::{OpenThread, THREAD_ALL_ACCESS, TerminateThread},
        },
    },
    core::{PCSTR, s},
};

type CreateThreadFn = unsafe extern "system" fn(
    lp_thread_attributes: *mut c_void,
    dw_stack_size: usize,
    lp_start_address: *mut c_void,
    lp_parameter: *mut c_void,
    dw_creation_flags: u32,
    lp_thread_id: *mut u32,
) -> HANDLE;

#[cfg(target_pointer_width = "64")]
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64 as IMAGE_NT_HEADERS;

#[cfg(target_pointer_width = "32")]
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32 as IMAGE_NT_HEADERS;

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

static INTEGRITY_SECTION_RANGE: LazyLock<Option<(usize, usize)>> = LazyLock::new(|| {
    let target = find_section_address_range(".UBX0")?;
    println!("found section @ {} - {}", target.start, target.end);
    return Some((target.start, target.end));
});

/// Extract the jump target address
fn jump_target_address(inst: &libmem::Inst) -> Option<usize> {
    let next_address = inst.address as i64 + inst.bytes.len() as i64;

    let target = match inst.bytes.as_slice() {
        [0xE9, displacement @ ..] if displacement.len() == 4 => {
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
        println!("analyzing thread {:X}...", start_address);

        let section_range = (*INTEGRITY_SECTION_RANGE)?;
        println!("- section range: {}, {}", section_range.0, section_range.1);

        let inst = libmem::disassemble(start_address)?;

        if inst.mnemonic.to_lowercase() != "jmp" {
            println!("- first inst was not a jump");
            return Some(false);
        }

        let target_addr = jump_target_address(&inst)?;
        println!("- jmp target addr: {}", target_addr);

        let in_range = target_addr >= section_range.0 && target_addr <= section_range.1;
        println!("verdict for thread {:X}: {}", start_address, in_range);

        return Some(in_range);
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
            return Err(format!(
                "failed to query thread information: {:?}",
                nt_status
            ));
        }

        if analyze_thread_start(thread_start_address) == Some(true) {
            TerminateThread(thread_handle, 0x0).map_err(|_| "failed to terminate thread")?;
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn terminate_integrity_checks() -> Result<(), String> {
    let process_id = libmem::get_process().unwrap().pid;
    let thread_list = libmem::enum_threads().ok_or("failed to enumerate threads")?;

    // Check all thread of the current process
    for thread in thread_list {
        if thread.owner_pid == process_id {
            let check_result = check_thread(thread.tid);
            match check_result {
                Ok(true) => {
                    println!("Terminated integrity check thread {}", thread.tid);
                }

                Err(e) => {
                    eprintln!("Error checking thread {}: {}", thread.tid, e);
                }

                _ => {}
            }
        }
    }

    Ok(())
}

pub struct IntegrityHook {
    original_func: Option<CreateThreadFn>,
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
        original_func: None,
        trampoline: None,
        target_address: create_thread_address,
    })
});

impl IntegrityHook {
    pub fn inst() -> MutexGuard<'static, IntegrityHook> {
        INSTANCE.lock().unwrap()
    }

    pub fn apply(&mut self) -> Result<(), String> {
        unsafe {
            let hook_address = Self::hk_create_thread as *mut c_void as usize;

            let trampoline = libmem::hook_code(self.target_address, hook_address)
                .ok_or("failed to hook CreateThread")?;

            self.original_func = trampoline.callable();
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
            lp_start_address = Self::empty_thread as *mut c_void;
            println!("CreateThread: prevented integrity check thread creation");
        }

        return unsafe {
            Self::inst().original_func.unwrap()(
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
