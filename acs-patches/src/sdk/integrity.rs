use std::{
    collections::HashMap,
    ffi::c_void,
    ops::Range,
    sync::{
        LazyLock, Mutex, MutexGuard, OnceLock, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use anyhow::{Context, Result, bail};
use framework::utils;
use windows::{
    Wdk::System::Threading::{NtQueryInformationThread, ThreadQuerySetWin32StartAddress},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Threading::{OpenThread, THREAD_ALL_ACCESS, TerminateThread},
        },
    },
    core::s,
};

/*
    The main integrity check thread immediately jumps into the VMP section (.UBX0).
    We can thus identify the MIC by analyzing the first instruction of every new
    thread to see if it is a jmp/call into said section.
*/

type CreateThreadFn = unsafe extern "system" fn(
    lp_thread_attributes: *mut c_void,
    dw_stack_size: usize,
    lp_start_address: *mut c_void,
    lp_parameter: *mut c_void,
    dw_creation_flags: u32,
    lp_thread_id: *mut u32,
) -> HANDLE;

static ORIG_CREATE_THREAD: OnceLock<CreateThreadFn> = OnceLock::new();

static INTEGRITY_SECTION_RANGE: LazyLock<Option<Range<usize>>> =
    LazyLock::new(|| utils::platform::find_section_address_range(".UBX0"));

static INTEGRITY_THREAD_VERDICTS: LazyLock<RwLock<HashMap<usize, bool>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static INTEGRITY_THREAD_FOUND: AtomicBool = AtomicBool::new(false);

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

            let target_addr = utils::resolve_relative_target(&inst)?;
            tracing::debug!("target addr of {}: {}", mnemonic, target_addr);

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

fn check_thread(thread_id: u32) -> Result<bool> {
    unsafe {
        let mut thread_start_address = 0x0;

        // Get a handle to the thread
        let thread_handle = OpenThread(THREAD_ALL_ACCESS, false, thread_id)?;

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
            bail!("failed to query thread information: {:?}", nt_status);
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
pub fn terminate_integrity_checks() -> Result<bool> {
    let process_id = libmem::get_process()
        .context("failed to get current process")?
        .pid;
    let thread_list = libmem::enum_threads().context("failed to enumerate threads")?;
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

pub fn initialize() -> Result<()> {
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
    utils::wait_until_true(Duration::from_secs(30), Duration::from_millis(10), || {
        INTEGRITY_THREAD_FOUND.load(Ordering::SeqCst)
    })
    .context("failed to wait for integrity check thread")?;

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

    pub fn apply(&mut self) -> Result<()> {
        if self.trampoline.is_some() {
            return Ok(());
        }

        unsafe {
            let hook_address = Self::hk_create_thread as *mut c_void as usize;

            let trampoline = libmem::hook_code(self.target_address, hook_address)
                .context("failed to hook CreateThread")?;

            let _ = ORIG_CREATE_THREAD.set(trampoline.callable::<CreateThreadFn>());
            self.trampoline = Some(trampoline);
        }

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        if let Some(trampoline) = self.trampoline.take() {
            unsafe {
                libmem::unhook_code(self.target_address, trampoline)
                    .context("failed to unhook CreateThread")?;
            }
        }

        Ok(())
    }

    extern "system" fn empty_thread(_: *mut c_void) -> u32 {
        0
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

        unsafe {
            ORIG_CREATE_THREAD.wait()(
                lp_thread_attributes,
                dw_stack_size,
                lp_start_address,
                lp_parameter,
                dw_creation_flags,
                lp_thread_id,
            )
        }
    }
}
