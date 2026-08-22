#![no_std]
#![no_main]

mod gdt;
mod printk;
mod vga;

use crate::vga::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    put_str("hello world - kfs kernel");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
