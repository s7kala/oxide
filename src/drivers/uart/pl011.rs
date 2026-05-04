use crate::arch::aarch64::mmio;
use crate::board::rpi4;
use oxide::expand_newlines;

mod regs {
    // UART register offsets
    pub const DR: usize = 0x00;
    pub const FR: usize = 0x18;
    pub const IBRD: usize = 0x24;
    pub const FBRD: usize = 0x28;
    pub const LCR_H: usize = 0x2C;
    pub const CR: usize = 0x30;
    pub const IMSC: usize = 0x38;
    pub const ICR: usize = 0x44;
}

mod flags {
    pub const TXFF: u32 = 1 << 5;
}

mod cfg {
    pub const WLEN_8: u32 = 0b11 << 5;
    pub const FEN: u32 = 1 << 4;
    pub const RXE: u32 = 1 << 9;
    pub const TXE: u32 = 1 << 8;
    pub const UARTEN: u32 = 1 << 0;
}

mod gpio {
    pub const GPFSEL1: usize = 0x04;
    pub const PUP_PDN_CNTRL_REG0: usize = 0xE4;

    pub const GPIO14_FSEL: u32 = 0b100 << 12;
    pub const GPIO15_FSEL: u32 = 0b100 << 15;
    pub const FSEL_CLEAR: u32 = (0b111 << 12) | (0b111 << 15);

    pub const PUP_PDN_CLEAR: u32 = 0b1111 << 28;
}

/// PL011 UART driver for Raspberry Pi 4 UART0.
pub struct Pl011Uart {
    base: usize,
}

impl Pl011Uart {
    /// Creates a new PL011Uart instance configured for UART0.
    pub const fn new() -> Self {
        Self {
            base: rpi4::UART0_BASE,
        }
    }

    /// Initializes the UART and GPIO pins for UART0 operation.
    /// Configures GPIO14/15 for ALT0 with disabled pulls, sets baud rate to 115200, enables 8-bit mode with FIFOs.
    pub fn init(&self) {
        unsafe {
            // Configure GPIO14/15 for UART0 ALT0 function
            let value = mmio::read32(rpi4::GPIO_BASE + gpio::GPFSEL1);
            let value = (value & !gpio::FSEL_CLEAR) | gpio::GPIO14_FSEL | gpio::GPIO15_FSEL;
            mmio::write32(rpi4::GPIO_BASE + gpio::GPFSEL1, value);

            // Disable GPIO pulls for GPIO14/15 (Pi 4 style)
            let value = mmio::read32(rpi4::GPIO_BASE + gpio::PUP_PDN_CNTRL_REG0);
            let value = value & !gpio::PUP_PDN_CLEAR;
            mmio::write32(rpi4::GPIO_BASE + gpio::PUP_PDN_CNTRL_REG0, value);

            // Disable UART before configuration
            mmio::write32(self.base + regs::CR, 0);
            // Clear all pending interrupts
            mmio::write32(self.base + regs::ICR, 0x7FF);
            // Disable all interrupts
            mmio::write32(self.base + regs::IMSC, 0);
            // Set integer baud rate divisor (115200 @ 48MHz assumed)
            mmio::write32(self.base + regs::IBRD, 26);
            // Set fractional baud rate divisor
            mmio::write32(self.base + regs::FBRD, 3);
            // Configure 8-bit mode with FIFOs enabled
            mmio::write32(self.base + regs::LCR_H, cfg::WLEN_8 | cfg::FEN);
            // Enable UART with TX and RX
            mmio::write32(self.base + regs::CR, cfg::UARTEN | cfg::TXE | cfg::RXE);
        }
    }

    /// Writes a single byte to the UART, blocking until the transmit FIFO has space.
    pub fn putc(&self, c: u8) {
        // Wait for transmit FIFO to have space
        while unsafe { mmio::read32(self.base + regs::FR) } & flags::TXFF != 0 {}

        unsafe {
            mmio::write32(self.base + regs::DR, c as u32);
        }
    }

    /// Writes a string to the UART, converting '\n' to '\r\n' for proper line endings.
    pub fn puts(&self, s: &str) {
        expand_newlines(s, |b| self.putc(b));
    }
}
