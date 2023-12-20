#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod multiboot;
pub mod io;
pub mod logger;
pub mod sys;
pub mod framebuffer;
pub mod mem;

#[cfg(target_arch = "x86")]
#[path="arch/i686/mod.rs"]
pub mod arch;

use multiboot::MultibootHeader;
use arch::gdt::gdt_init;
use core::arch::asm;
use arch::interrupts::interrupts_init;
use arch::pic::PICS;
use sys::syscalls::syscalls_init;
use framebuffer::{russian_flag, Framebuffer};
use raw_cpuid::CpuId;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    err!("{}", info);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(multiboot_addr: u32, _stack_top: u32) -> ! {
    let mboot: *const MultibootHeader = multiboot_addr as *const MultibootHeader;

    log!("Swan OS starting...");
    log!("Multiboot header at {:?}", mboot);

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

    let mut fb = Framebuffer::new(mboot);
    russian_flag(&mut fb);

    gdt_init();
    interrupts_init();
    PICS.lock().init();
    syscalls_init();

    log!("Test printable syscall");
    asm!("int $0x80", in("eax") (0), in("ebx") (12));

    arch::hlt()
}