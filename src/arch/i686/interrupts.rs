use crate::arch::idt::InterruptDescriptorTable;
use crate::arch::registers::Registers;
use lazy_static::lazy_static;
use crate::arch::hlt;
use spin::Mutex;
use x86::io;
use crate::{log, ok};

lazy_static! {
    pub static ref IDT: Mutex<InterruptDescriptorTable> = {
        let mut idt = InterruptDescriptorTable::new();
        idt.init();
        Mutex::new(idt)
    };
}

#[no_mangle]
unsafe extern "C" fn irq_handler(regs: Registers) {
    if regs.int_num >= 40 {
		io::outb(0xA0, 0x20);
	}
    io::outb(0x20, 0x20);

    let int_num = regs.int_num as usize;
    if IDT.lock().interrupt_handlers[int_num] != None {
        let handler = IDT.lock().interrupt_handlers[int_num].unwrap();
        handler(regs);
    }
}

#[no_mangle]
unsafe extern "C" fn isr_handler(regs: Registers) {
    let int_num = regs.int_num as usize;
    if IDT.lock().interrupt_handlers[int_num] != None {
        let handler = IDT.lock().interrupt_handlers[int_num].unwrap();
        handler(regs);
    }
}

fn divide_by_zero(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION DIVISION BY ZERO (0x0)");
    hlt();
    
}

fn fault_opcode(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION FAULT OPCODE (0x6)");
    hlt();
}

fn double_fault(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION DOUBLE FAULT (0x8)");
    hlt();
}

fn segment_is_not_available(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION SEGMENT ISN'T AVAILABLE (0xB)");
    hlt();
}

fn stack_error(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION STACK ERR (0xC)");
    hlt();
}

fn general_protection_error(_regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION GENERAL PROTECTION ERR (0xD)");
    hlt();
}

fn page_fault(regs: Registers) {
    log!("CRITICAL ERROR: EXCEPTION PAGE FAULT (0xE)");
    log!("Error code: {}", regs.err_code);
    hlt();
}


pub fn interrupts_init() {
    IDT.lock().register_interrupt_handler(0x0, divide_by_zero);
    IDT.lock().register_interrupt_handler(0x6, fault_opcode);
    IDT.lock().register_interrupt_handler(0x8, double_fault);
    IDT.lock().register_interrupt_handler(0xB, segment_is_not_available);
	IDT.lock().register_interrupt_handler(0xC, stack_error);
	IDT.lock().register_interrupt_handler(0xD, general_protection_error);
	IDT.lock().register_interrupt_handler(0xE, page_fault);
    ok!("Interrupt handers has been initializated!");
}