extern crate windows;

use core::ptr;

// 4KB page size
pub fn page_size() -> usize { 4 * 1024 }

pub unsafe fn get_preinstalled_memory() -> (usize, usize) { (0, 0) }

pub unsafe fn alloc(size: usize) -> (*mut u8, usize, u32) {
    let addr = windows::VirtualAlloc(
    
    );
}