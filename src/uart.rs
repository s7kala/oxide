pub const UART0_BASE: usize = 0xfe20_1000;

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new() -> Self {
        Self { base: UART0_BASE }
    }

    pub const fn base(&self) -> usize {
        self.base
    }
}
