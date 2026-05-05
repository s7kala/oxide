use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::io::console::panic_console_init();

    println!();
    println!("KERNEL PANIC");
    println!("{}", info);

    loop {
        core::hint::spin_loop();
    }
}
