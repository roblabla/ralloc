//! System calls.

/// Change the data segment. See `man brk`.
///
/// On success, the new program break is returned. On failure, the old program break is returned.
///
/// # Note
///
/// This is the `brk` **syscall**, not the library function.
#[cfg(not(target_os = "horizon"))]
pub unsafe fn brk(ptr: *const u8) -> *const u8 {
    syscall!(BRK, ptr) as *const u8
}

/// Change the data segment. See `man brk`.
///
/// On success, the new program break is returned. On failure, the old program break is returned.
///
/// # Note
///
/// This is the `brk` **syscall**, not the library function.
#[cfg(target_os = "horizon")]
pub unsafe fn brk(ptr: *const u8) -> *const u8 {
    use libtransistor_sys::*;
    use core::ptr;

    fn nearest_multiple(n: u64, mult: u64) -> u64 {
        let rem = n % mult;
        if rem != 0 {
            n + mult - rem
        } else {
            n
        }
    }

    static mut CUR_HEAP_SIZE: u64 = 0;

    const RESERVED_HEAP_REGION_BASE_ADDR: u64 = 4;
    const RESERVED_HEAP_REGION_SIZE: u64 = 5;

    let mut base_addr = 0;
    let ret = svcGetInfo(&mut base_addr, RESERVED_HEAP_REGION_BASE_ADDR, CURRENT_PROCESS, 0);
    if ret != 0 {
        return ptr::null();
    }
    if ptr.is_null() {
        (base_addr + CUR_HEAP_SIZE) as _
    } else {
        // TODO: Make sure we don't send a negative number in there.
        // TODO: Might not need to call setHeapSize if nearest_multiple(CUR_HEAP_SIZE) ==
        // nearest_multiple(size_needed)
        let size_needed = (ptr as u64) - base_addr;
        let new_size = nearest_multiple(size_needed, 0x2000000);
        let mut out_addr = ptr::null_mut();
        let ret = svcSetHeapSize(&mut out_addr, new_size as u32);
        if ret == 0 {
            CUR_HEAP_SIZE = size_needed;
        }
        (out_addr as *const u8).offset(size_needed as isize) as _
    }
}

/// Voluntarily give a time slice to the scheduler.
#[cfg(not(target_os = "horizon"))]
pub fn sched_yield() -> usize {
    unsafe { syscall!(SCHED_YIELD) }
}

/// Voluntarily give a time slice to the scheduler.
#[cfg(target_os = "horizon")]
pub fn sched_yield() -> usize {
    use libtransistor_sys::*;
    unsafe { svcSleepThread(0) as usize }
}
