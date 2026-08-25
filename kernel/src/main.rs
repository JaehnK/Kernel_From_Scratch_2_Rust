#![no_std]
#![no_main]

mod dump_stack;
mod gdt;
mod printk;
mod vga;

use crate::vga::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    vga::put_str("Welcome to KFS2 kernel\n");

    printk::vprintk(
        "%s: %p, %d%%\n",
        &[
            printk::Arg::Str("Print-test:"),
            printk::Arg::Hex(0xdeadbeef),
            printk::Arg::Int(-42),
        ],
    );

    dump_stack::dump_stack();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
