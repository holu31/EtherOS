use core::arch::asm;

#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut tmp: u8 = 0;
    unsafe {
        asm!("in al, dx", out("al") tmp, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    tmp
}


#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let mut tmp: u16 = 0;
    unsafe {
        asm!("in ax, dx", out("ax") tmp, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    tmp
}

#[inline]
pub unsafe fn outl(port: u16, value: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let mut tmp: u32 = 0;
    unsafe {
        asm!("in dx, eax", in("dx") port, out("eax") tmp, options(nomem, nostack, preserves_flags));
    }
    tmp
}

pub fn io_wait() {
    unsafe {
        outb(0x80, 0);
    }
}