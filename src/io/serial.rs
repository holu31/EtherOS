use core::{fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::io::ports::{inb, outb};

pub static PORT_COM1: u16 = 0x3f8;
pub static PORT_COM2: u16 = 0x2F8;
pub static PORT_COM3: u16 = 0x3E8;
pub static PORT_COM4: u16 = 0x2E8;
pub static PORT_COM5: u16 = 0x5F8;
pub static PORT_COM6: u16 = 0x4F8;
pub static PORT_COM7: u16 = 0x5E8;
pub static PORT_COM8: u16 = 0x4E8;

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(PORT_COM1);
        unsafe { serial_port.init(); }
        Mutex::new(serial_port)
    };
}

pub struct SerialPort {
    port: u16
}

impl SerialPort {

    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub unsafe fn init(&mut self) -> bool {
        outb(self.port + 1, 0x00);
        outb(self.port + 3, 0x80);
        outb(self.port + 0, 0x03);
        outb(self.port + 1, 0x00);
        outb(self.port + 3, 0x03);
        outb(self.port + 2, 0xC7);
        outb(self.port + 4, 0x0B);
        outb(self.port + 4, 0x1E);
        outb(self.port + 0, 0xAE);

        if inb(self.port + 0) != 0xAE {
            return false;
        }

        outb(self.port + 4, 0x0F);

        true
    }

    unsafe fn is_transmit_empty(&self) -> u8 {
        inb(self.port + 5) & 0x20
    }

    pub fn send_byte(&mut self, c: u8) {
        unsafe { 
            while self.is_transmit_empty() == 0 {}

            outb(self.port, c)
        }
    }

    pub fn send(&mut self, s: &str) {
        for b in s.bytes() {
            self.send_byte(b);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.send(s);

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    COM1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::io::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}