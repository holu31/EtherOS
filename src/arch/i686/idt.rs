use core::mem::size_of;
use crate::arch::registers::Registers;
use crate::{ok, log};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    base_lo: u16,
    sel: u16,
    always0: u8,
    flags: u8,
    base_hi: u16
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: *const IdtEntry
}

pub struct InterruptDescriptorTable {
    entries: [IdtEntry; 256],
    pub interrupt_handlers: [Option<fn(Registers)>; 256]
}

impl InterruptDescriptorTable {
    
    pub fn new() -> Self {
        let entries = [IdtEntry {
            base_lo: 0,
            sel: 0,
            always0: 0,
            flags: 0,
            base_hi: 0
        }; 256];

        Self { entries, interrupt_handlers: [None; 256] }
    }

    pub fn set_gate(&mut self, index: usize, base: u32, sel: u16, flags: u8) {
        let idt_entry = IdtEntry {
            base_lo: (base & 0xFFFF) as u16,
            sel,
            always0: 0,
            flags: flags | 0x60,
            base_hi: (base >> 16) as u16 & 0xFFFF,
        };

        self.entries[index] = idt_entry;
    }

    pub fn register_interrupt_handler(&mut self, n: u8, handler: fn(Registers) -> ()) {
        log!("Register handler for INTERRUPT {}", n);
        self.interrupt_handlers[n as usize] = Some(handler);
    }

    pub fn init(&mut self) {
        let table = [
            _isr0, _isr1, _isr2, _isr3, _isr4, _isr5, _isr6, _isr7,
            _isr8, _isr9, _isr10, _isr11, _isr12, _isr13, _isr14, _isr15,
            _isr16, _isr17, _isr18, _isr19, _isr20, _isr21, _isr22, _isr23,
            _isr24, _isr25, _isr26, _isr27, _isr28, _isr29, _isr30, _isr31,
            _irq0, _irq1, _irq2, _irq3, _irq4, _irq5, _irq6, _irq7, _irq8, _irq9,
            _irq10, _irq11, _irq12, _irq13, _irq14, _irq15
        ];

        for i in 0..48 {
            self.set_gate(i, table[i] as u32, 0x08, 0xEF);
        }

        self.set_gate(0x80, _isr128 as u32, 0x08, 0xEF);

        let idt_ptr = IdtPtr {
            limit: (256 * size_of::<IdtEntry>() - 1) as u16,
            base: self.entries.as_ptr()
        };

        unsafe {
            idt_load(&idt_ptr as *const IdtPtr);
        }
        ok!("Interrupt Description Table has been initializated!");
    }
}

extern {
    fn idt_load(idt_ptr: *const IdtPtr);

    fn _irq0();
    fn _irq1();
    fn _irq2();
    fn _irq3();
    fn _irq4();
    fn _irq5();
    fn _irq6();
    fn _irq7();
    fn _irq8();
    fn _irq9();
    fn _irq10();
    fn _irq11();
    fn _irq12();
    fn _irq13();
    fn _irq14();
    fn _irq15();

    fn _isr0();
    fn _isr1();
    fn _isr2();
    fn _isr3();
    fn _isr4();
    fn _isr5();
    fn _isr6();
    fn _isr7();
    fn _isr8();
    fn _isr9();
    fn _isr10();
    fn _isr11();
    fn _isr12();
    fn _isr13();
    fn _isr14();
    fn _isr15();
    fn _isr16();
    fn _isr17();
    fn _isr18();
    fn _isr19();
    fn _isr20();
    fn _isr21();
    fn _isr22();
    fn _isr23();
    fn _isr24();
    fn _isr25();
    fn _isr26();
    fn _isr27();
    fn _isr28();
    fn _isr29();
    fn _isr30();
    fn _isr31();
    fn _isr128();
}