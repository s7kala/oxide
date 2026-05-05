use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::io::console::panic_console_init();

    crate::io::console::panic_print(format_args!("\nKERNEL PANIC\n{}\n", info));

    loop {
        core::hint::spin_loop();
    }
}
