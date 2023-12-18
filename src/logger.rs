#[macro_export]
macro_rules! log {
    ($prefix:expr, $color:expr, $($arg:tt)*) => {
        let file = file!();
        let line = line!();

        $crate::serial_print!("[\x1b[{};1m{}\x1b[{};0m] [rust/{}:{}]\x1b[{};1m {} \x1b[{};0m\n",
            $color, $prefix, $color,
            file,
            line,
            $color,
            format_args!($($arg)*),
            $color
        )
    };
    ($($arg:tt)*) => {
        $crate::log!("LOG", "0", $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!("WARN", "33", $($arg)*)
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::log!("ERROR", "31", $($arg)*)
    };
}

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        $crate::log!("OK", "32", $($arg)*)
    };
}

#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {
        $crate::log!("NOTE", "36", $($arg)*)
    };
}