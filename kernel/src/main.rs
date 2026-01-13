#![no_std]
#![no_main]

mod arch;
mod boot;
mod io;
mod mm;

use core::panic::PanicInfo;

use crate::boot::limine;
use crate::mm::frame;
use crate::arch::x86_64::{gdt, idt, serial};

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    serial::init();
    println!("Initialized serial");

    gdt::init();
    println!("Initialized gdt");

    idt::init();
    println!("Initialized idt");

    let regions = limine::build_kernel_memory_map();
    println!("Built kernel memory map");

    let hhdm = limine::get_hhdm_offset();
    println!("Got higher-half direct map offset");

    frame::init(regions, hhdm);
    println!("Initialized frame allocator");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("KERNEL PANIC!");
    println!("{}", info);
    loop {}
}
