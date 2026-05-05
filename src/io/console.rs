use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::drivers::uart::pl011::Pl011Uart;

static UART: Pl011Uart = Pl011Uart::new();
static UART_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn init() {
    if UART_INITIALIZED.load(Ordering::Acquire) {
        return;
    }

    UART.init();
    UART_INITIALIZED.store(true, Ordering::Release);
}

pub fn panic_console_init() {
    init();
}

pub fn puts(s: &str) {
    init();
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
