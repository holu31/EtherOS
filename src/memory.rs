use core::mem::size_of;
use core::slice;
use core::ptr::null_mut;

// use spin::Mutex;
use crate::multiboot::{MultibootHeader, MemoryMapEntry};

use crate::{log, _kernel_end, _kernel_start};

pub unsafe fn init(mboot: *const MultibootHeader) {
    let mentry = (*mboot).mmap_addr as *mut MemoryMapEntry;
    let mut memory_map = MemoryMap::new(mentry, (*mboot).mmap_length);

    let kernel_size = _kernel_end - _kernel_start;

    log!("Kernel size is: {} bytes", kernel_size);

    memory_map.free_availble_memory(mboot);

	memory_map.bitmap[0] = 0xFFFF_FFFF;

    let a = memory_map.alloc_page();
    let b = memory_map.alloc_page();
    let c = memory_map.alloc_page();

    if a == null_mut() {
        log!("нихуя не вышло!");
    }

    log!("Память выделена, адрес: {:?}", a);
    log!("Память выделена, адрес: {:?}", b);
    log!("Память выделена, адрес: {:?}", c);
}

pub struct MemoryMap<'a> {
    maddr: *mut MemoryMapEntry,
    available_memory: usize,
    max_memory: usize,
    pages_count: usize,
    used: isize,
    bitmap: &'a mut [u32]
}

impl<'a> MemoryMap<'a> {
    pub unsafe fn new(maddr: *mut MemoryMapEntry, length: u32) -> Self {
        let mut available_memory = 0;
        let mut max_memory = 0;

        for i in 0..(length as usize / size_of::<MemoryMapEntry>()) as isize {
            if (*(maddr.offset(i))).type_ == 1 {
                available_memory += (*(maddr.offset(i))).len as usize;
            }

            max_memory += (*(maddr.offset(i))).len as usize;
        }

        log!("Available memory {} KB", available_memory >> 10);
        log!("Max memory {} KB", max_memory >> 10);

        let pages_count = max_memory / 4096;
        let bitmap = slice::from_raw_parts_mut(_kernel_end as *mut u32, pages_count / 8);

        bitmap.fill(0xFFFF_FFFF);

        log!("Total pages count: {}", pages_count);

        Self {
            maddr,
            available_memory,
            max_memory,
            pages_count,
            used: 0,
            bitmap
        }
    }

    unsafe fn free_availble_memory(&mut self, mboot: *const MultibootHeader) {
        let mut mentry = (*mboot).mmap_addr as *mut MemoryMapEntry;
        while (mentry as usize) < (*mboot).mmap_addr as usize + (*mboot).mmap_length as usize {
            mentry = (mentry as usize + (*mentry).size as usize + size_of::<u8>()) as *mut MemoryMapEntry;
        }
    }

    pub fn alloc_page(&mut self) -> *mut u8 {
        for i in 0..self.pages_count {
            let idx = i / 32;
            let bit = i % 32;

            if self.bitmap[idx] & (1 << bit) != 0 {
                self.bitmap[idx] &= !(1 << bit);
                self.used += 1;
                
                return (i * 4096) as *mut u8;
            }
        }
        null_mut()
    }
}
