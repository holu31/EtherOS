use core::mem::size_of;
use crate::ok;

extern {
    fn gdt_flush(gdtr: *const GdtReg);
}

#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {

    pub const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            granularity: ((limit >> 16) & 0x0F) as u8 | gran & 0xF0,

            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            base_high: ((base >> 24) & 0xFF) as u8,

            access,
        }
    }

}

#[repr(C, packed)]
struct GdtReg {
    limit: u16,
    base: *const GdtEntry
}

static mut GDT: [GdtEntry; 5] = [
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFFFFF, 0x9A, 0xCF), // code segment
    GdtEntry::new(0, 0xFFFFFFFF, 0x92, 0xCF), // data segment
    GdtEntry::new(0, 0xFFFFFFFF, 0xFA, 0xCF), // system segment
    GdtEntry::new(0, 0xFFFFFFFF, 0xF2, 0xCF), // TSS
];

pub unsafe fn gdt_init() {
    let gdtr = GdtReg {
        limit: (GDT.len() * size_of::<GdtEntry>() - 1) as u16,
        base: GDT.as_ptr()
    };

    gdt_flush(&gdtr as *const GdtReg);
    ok!("Global Description Table has been initializated!");
}