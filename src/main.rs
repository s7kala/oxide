#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

mod panic;
mod uart;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let uart = uart::Uart::new();
    let _uart_base = uart.base();

    loop {
        core::hint::spin_loop();
    }
}
