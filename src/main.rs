#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

mod arch;
mod board;
mod drivers;
mod io;
mod panic;
mod startup;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    io::console::init();

    startup::diagnostics::print();

    loop {
        core::hint::spin_loop();
    }
}
