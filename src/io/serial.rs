use core::{fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

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
        let mut serial_port = unsafe { SerialPort::new(PORT_COM1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
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


/*


*/