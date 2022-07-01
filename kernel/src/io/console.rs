use core::fmt::Arguments;

#[macro_export]
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    };
}

#[macro_export]
macro_rules! print {
    ($color:expr;$($arg:tt)*) => (crate::io::console::print_in_color($color, format_args!($($arg)*)));
    ($($arg:tt)*) => (crate::io::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!(36; "[Info] :# {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! test {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!(96; "[-OK-] :) {} \n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!(31; "[Warn] :! {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!(93; "[Debug] :? {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!(91; "[error] :< {}\n", format_args!($($arg)*)));
}

pub fn print(fmt: Arguments) {
    crate::io::serial::putfmt(fmt);
}

pub fn print_in_color(color: u8, fmt: Arguments) {
    crate::io::serial::putfmt(with_color!(fmt, color));
}

