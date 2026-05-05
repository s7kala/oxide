use crate::println;

const KERNEL_LOAD_ADDR: usize = 0x80000;

unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
    static __boot_stack_top: u8;
}

#[inline(always)]
fn current_exception_level() -> u8 {
    let current_el: u64;

    unsafe {
        core::arch::asm!(
            "mrs {current_el}, CurrentEL",
            current_el = out(reg) current_el,
            options(nomem, nostack)
        );
    }

    ((current_el >> 2) & 0b11) as u8
}

pub fn print() {
    println!("Oxide");
    println!("current EL: {}", current_exception_level());
    println!(
        "__bss_start:      {:#018x}",
        &raw const __bss_start as usize
    );
    println!("__bss_end:        {:#018x}", &raw const __bss_end as usize);
    println!(
        "__boot_stack_top: {:#018x}",
        &raw const __boot_stack_top as usize
    );
    println!("kernel load addr: {:#018x}", KERNEL_LOAD_ADDR);
    println!("uart initialized");
}
