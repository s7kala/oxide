use crate::arch::aarch64::mmio;
use crate::board::rpi4;

pub struct Pl011Uart {
    base: usize,
}

impl Pl011Uart {
    pub const fn new() -> Self {
        Self {
            base: rpi4::UART0_BASE,
        }
    }

    pub fn init(&self) {
        // Rely on firmware / boot environment for early UART setup.
    }

    pub fn putc(&self, c: u8) {
        const UARTDR: usize = 0x00;
        const UARTFR: usize = 0x18;
        const TXFF: u32 = 1 << 5;

        while unsafe { mmio::read32(self.base + UARTFR) } & TXFF != 0 {}

        unsafe {
            mmio::write32(self.base + UARTDR, c as u32);
        }
    }

    pub fn puts(&self, s: &str) {
        for b in s.bytes() {
            if b == b'\n' {
                self.putc(b'\r');
            }
            self.putc(b);
        }
    }
}
