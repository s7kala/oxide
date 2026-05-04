use crate::drivers::uart::pl011::Pl011Uart;

static UART: Pl011Uart = Pl011Uart::new();

pub fn init() {
    UART.init();
}

pub fn putc(c: u8) {
    UART.putc(c);
}

pub fn puts(s: &str) {
    UART.puts(s);
}
