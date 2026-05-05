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
    println!("oxide");
    println!("uart initialized");

    loop {
        core::hint::spin_loop();
    }
}
