pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod registers;
pub mod pic;

use core::arch::asm;

pub fn hlt() -> ! {
    loop {
        unsafe { asm!("hlt"); }
    }
}