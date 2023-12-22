#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod multiboot;
pub mod io;
pub mod logger;
pub mod sys;
pub mod framebuffer;
pub mod memory;

#[cfg(target_arch = "x86")]
#[path="arch/i686/mod.rs"]
pub mod arch;

use multiboot::MultibootHeader;
use arch::gdt::gdt_init;
use core::arch::asm;
use arch::interrupts::interrupts_init;
use arch::pic::PICS;
use sys::syscalls::syscalls_init;
use raw_cpuid::CpuId;

extern {
    pub static _kernel_start: usize;
    pub static _kernel_end: usize;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    err!("{}", info);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(multiboot_addr: u32, _stack_top: u32) -> ! {
    let mboot: *const MultibootHeader = multiboot_addr as *const MultibootHeader;

    log!("Ether OS v{} starting...", env!("CARGO_PKG_VERSION"));
    log!("Multiboot header at {:?}", mboot);

    log!("Kernel start {:#x}", _kernel_start);
    log!("Kernel end {:#x}", _kernel_end);

    let cpuid = CpuId::new();
    if let Some(vendor) = cpuid.get_vendor_info() {
        log!("Loaded with CPU {}", vendor.as_str());
    }

    let has_sse = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_sse());
    if has_sse {
        log!("CPU supports SSE!");
    }

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
            note!("L{}-Cache size is {}", cache.level(), size);
        }
    } else {
        log!("No cache parameter information available");
    }

    gdt_init();
    interrupts_init();
    PICS.lock().init();
    syscalls_init();

    log!("Test printable syscall");
    asm!("int $0x80", in("eax") (0), in("ebx") (12));
    
    memory::init(mboot);

    arch::hlt();
}