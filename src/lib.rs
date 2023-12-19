#![no_std]
#![no_main]

use core::{panic::PanicInfo, fmt::Write};

pub mod multiboot;
pub mod io;
pub mod logger;
pub mod sys;
pub mod framebuffer;

#[cfg(target_arch = "x86")]
#[path="arch/i686/mod.rs"]
pub mod arch;

use multiboot::MultibootHeader;
use arch::gdt::gdt_init;
use core::arch::asm;
use arch::interrupts::interrupts_init;
use arch::pic::PICS;
use sys::syscalls::syscalls_init;
use framebuffer::Framebuffer;

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

    let mut fb = Framebuffer::new(mboot);
    for x in 0..1280 {
        for y in 0..720/3 {
            fb.pixel(x, y, 0xFFFFFF);
        }
    }

    for x in 0..1280 {
        for y in 720/3..720/2+100 {
            fb.pixel(x, y, 0x0000FF);
        }
    }

    for x in 0..1280 {
        for y in 720/2+100..720 {
            fb.pixel(x, y, 0xFF0000);
        }
    }

    fb.write_str("Hello, Russia!\n");
    fb.write_str("Russian font is not supported yet!");

    gdt_init();
    interrupts_init();
    PICS.lock().init();
    syscalls_init();

    log!("Test printable syscall");
    asm!("int $0x80", in("eax") (0), in("ebx") (12));

    arch::hlt()
}