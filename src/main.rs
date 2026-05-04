#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

mod arch;
mod board;
mod drivers;
mod io;
mod panic;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    io::console::init();
    io::console::puts("hello from oxide\n");

    loop {
        core::hint::spin_loop();
    }
}
