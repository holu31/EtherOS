use crate::arch::interrupts::IDT;
use crate::arch::registers::Registers;
use crate::{ok, log};

const PRINT: usize = 0;

fn syscalls_handler(regs: Registers) {
    log!("Called syscall, EAX = {}", regs.eax);
    match regs.eax as usize {
        PRINT => {
            log!("Result: {}", regs.ebx);
        },
        _ => {
            log!("Exception: Unknown syscall called!");
        }
    }
}

pub fn syscalls_init() {
    IDT.lock().register_interrupt_handler(0x80, syscalls_handler);
    ok!("Syscall has been initializated!");
}