use crate::dlassert;
use crate::dlverbose;
use core::ptr;
use core::mem;
use core::ffi::{c_void};
use once_cell::sync::Lazy;

use windows::Win32::System;

pub fn page_size() -> usize { 4096 }

pub unsafe fn get_preinstalled_memory() -> (usize, usize) { (0, 0) }

pub unsafe fn alloc(size: usize) -> (*mut u8, usize, u32) {
    let addr = windows::Win32::System::Memory::VirtualAlloc(
        ptr::null_mut(),
        size,
        System::Memory::MEM_RESERVE | System::Memory::MEM_COMMIT,
        System::Memory::PAGE_READWRITE,
    );

    if addr == ptr::null_mut() {
        (ptr::null_mut(), 0, 0)
    } else {
        (addr as *mut u8, size, 0)
    }
}

pub unsafe fn free(ptr: *mut u8, size: usize) -> bool {
    System::Memory::VirtualFree(
        ptr as *mut c_void,
        0,
        windows::Win32::System::Memory::MEM_RELEASE).0 != 0
}

pub use crate::common::get_free_borders;

#[cfg(feature = "global")]
static LOCK: Lazy<windows::Win32::Foundation::HANDLE> = unsafe {
    Lazy::new(|| {
        windows::Win32::System::Threading::CreateMutexA(
            ptr::null_mut(),
            false,
            windows::core::PCSTR::default())
    })
};


#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    let result = unsafe { windows::Win32::System::Threading::WaitForSingleObject(*LOCK, u32::MAX) };
    assert_ne!(result, u32::MAX);
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    let result = unsafe { windows::Win32::System::Threading::ReleaseMutex(*LOCK).0 };
    assert_ne!(result, 0);
}
