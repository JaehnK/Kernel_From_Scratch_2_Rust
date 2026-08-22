#![no_std]
#![no_main]

mod gdt;
mod printk;
mod vga;

use crate::vga::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    vga::put_str("Welcome to KFS2 kernel");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
