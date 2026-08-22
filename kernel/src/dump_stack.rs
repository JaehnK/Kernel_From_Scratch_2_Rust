use crate::printk;
use crate::printk::Arg;

pub fn dump_stack() {
    let esp: u32;
    unsafe { core::arch::asm!("mov {}, esp", out(reg) esp) };

    printk!("ESP: %p\n", Arg::Hex(esp));
    for row in 0..4 {
        // 4워드 × 4행 = 64바이트
        let addr = esp + row * 16;
        printk!("%p: ", Arg::Hex(addr));
        for w in 0..4 {
            let val = unsafe { *((addr + w * 4) as *const u32) };
            printk!("%p ", Arg::Hex(val));
        }
        printk!("\n");
    }
}
