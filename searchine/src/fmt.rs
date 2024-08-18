/// Prints text in bold red.
#[macro_export]
macro_rules! println_red {
    ($($arg:tt)*) => {
        println!("\x1b[31m\x1b[1m{}\x1b[0m", format!($($arg)*));
    };
}

/// Prints text in bold green
#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => {
        println!("\x1b[32m\x1b[1m{}\x1b[0m", format!($($arg)*));
    };
}

/// Prints text in bold.
#[macro_export]
macro_rules! println_bold {
    ($($arg:tt)*) => {
        println!("\x1b[1m{}\x1b[0m", format!($($arg)*));
    };
}
