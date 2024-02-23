#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Info,
    Debug,
    Warn,
    Error,
}

#[macro_export]
macro_rules! log_level {
    ($level:expr, $($arg:tt)+) => {
        if cfg!(debug_assertions) {
            $crate::println!(
                "\x1B[{}m[{:^5}] {}\x1B[0m", 
                $crate::init::log::log_color($level), $level, format_args!($($arg)+)
            );
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log_level!($crate::init::log::Level::Info, $($arg)+))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log_level!($crate::init::log::Level::Debug, $($arg)+))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crateinit::log::log_level!($crate::init::log::Level::Warn, $($arg)+))
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crateinit::log::log_level!($crate::init::log::Level::Error, $($arg)+))
}

pub fn log_color(level: Level) -> &'static str {
    match level {
        Level::Info => "32",  // Green
        Level::Debug => "36", // Cyan
        Level::Warn => "33",  // Yellow
        Level::Error => "31", // Red
    }
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Info => f.write_str("INFO"),
            Self::Debug => f.write_str("DEBUG"),
            Self::Warn => f.write_str("WARN"),
            Self::Error => f.write_str("ERROR"),
        }
    } 
}
