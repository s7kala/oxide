use core::fmt::{self, Write};

use crate::drivers::uart::pl011::Pl011Uart;

static UART: Pl011Uart = Pl011Uart::new();

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn init() {
    UART.init();
}

pub fn puts(s: &str) {
    UART.puts(s);
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    let mut console = Console;
    let _ = console.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::io::console::_print(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*);
    };
}
