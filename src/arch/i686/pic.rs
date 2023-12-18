use crate::io::ports::{outb, inb, io_wait};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::ok;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = 0xA0;

lazy_static! {
    pub static ref PICS: Mutex<Pics> = {
        let mut pics = Pics::new(PIC_1_OFFSET, PIC_2_OFFSET);
        Mutex::new(pics)
    };
}

const PCI_INIT: u8 = 0x11;
const PIC_EOI: u8 = 0x20;
const MODE_8086: u8 = 0x01;

struct Pic {
    pub offset: u8,
    pub command: u8,
    pub data: u8,
}

impl Pic {
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    pub unsafe fn end_of_interrupt(&mut self) {
        outb(self.command.into(), PIC_EOI);
    }

    pub unsafe fn read_mask(&mut self) -> u8 {
        inb(self.data.into())
    }

    pub unsafe fn write_mask(&mut self, mask: u8) {
        outb(self.data.into(), mask);
    }
}

pub struct Pics {
    pics: [Pic; 2]
}

impl Pics {
    
    pub fn new(offset1: u8, offset2: u8) -> Self {
        Self {
            pics: [
                Pic {
                    offset: offset1,
                    command: 0x20,
                    data: 0x21
                },
                Pic {
                    offset: offset2,
                    command: 0xA0,
                    data: 0xA1
                }
            ]
        }
    }

    pub unsafe fn init(&mut self) {
        let saved_masks = self.read_masks();

        outb(self.pics[0].command.into(), PCI_INIT);
        io_wait();
        outb(self.pics[1].command.into(), PCI_INIT);
        io_wait();

        outb(self.pics[0].data.into(), self.pics[0].offset);
        io_wait();
        outb(self.pics[1].data.into(), self.pics[1].offset);
        io_wait();

        outb(self.pics[0].data.into(), 4);
        io_wait();
        outb(self.pics[1].data.into(), 2);
        io_wait();

        outb(self.pics[0].data.into(), MODE_8086);
        io_wait();
        outb(self.pics[1].data.into(), MODE_8086);
        io_wait();
        
        self.write_masks(saved_masks[0], saved_masks[1]);
        ok!("PIC8259 has been initializated!");
    }

    pub unsafe fn disable(&mut self) {
        self.write_masks(u8::MAX, u8::MAX);
    }

    pub unsafe fn read_masks(&mut self) -> [u8; 2] {
        [self.pics[0].read_mask(), self.pics[1].read_mask()]
    }

    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].write_mask(mask1);
        self.pics[1].write_mask(mask2);
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }
}