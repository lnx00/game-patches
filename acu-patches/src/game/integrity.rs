use std::{
    ffi::c_void,
    sync::{LazyLock, Mutex, MutexGuard},
};

use windows::{
    Wdk::System::Threading::{NtQueryInformationThread, ThreadQuerySetWin32StartAddress},
    Win32::{
        Foundation::HANDLE,
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Threading::{OpenThread, THREAD_ALL_ACCESS, TerminateThread},
        },
    },
    core::s,
};

type CreateThreadFn = unsafe extern "system" fn(
    lp_thread_attributes: *mut c_void,
    dw_stack_size: usize,
    lp_start_address: *mut c_void,
    lp_parameter: *mut c_void,
    dw_creation_flags: u32,
    lp_thread_id: *mut u32,
) -> HANDLE;

const INTEGRITY_THREAD_START_ADDRESS: usize = 0x14275DE50;

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

        if thread_start_address == INTEGRITY_THREAD_START_ADDRESS {
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

        if lp_start_address as usize == INTEGRITY_THREAD_START_ADDRESS {
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
