#[path = "../../kernel/src/gdt.rs"]
mod gdt;

use gdt::{Descriptor, GlobalDescriptorTable};

static GDT: GlobalDescriptorTable = GlobalDescriptorTable::new([
    Descriptor::new(0, 0, 0, 0),
    Descriptor::new(0, 0xFFFFF, 0x9A, 0xC),
    Descriptor::new(0, 0xFFFFF, 0x92, 0xC),
    Descriptor::new(0x11223344, 0x33333, 0x11, 0x1),
    Descriptor::new(0x55667788, 0x55555, 0x22, 0x2),
    Descriptor::new(0x99AABBCC, 0x77777, 0x33, 0x3),
    Descriptor::new(0xDDEEFF00, 0x99999, 0x44, 0x4),
]);

fn main() {
    println!("{:p}", &GDT);
}
