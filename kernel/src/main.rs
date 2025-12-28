#![no_std]
#![no_main]

mod arch;
mod boot;
mod io;
mod mm;

use crate::boot::limine;
use crate::mm::frame_alloc;
use crate::arch::x86_64::serial;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    serial::init();
    println!("Initialized serial");

    let regions = limine::build_kernel_memory_map();
    println!("Built kernel memory map");

    let hhdm = limine::get_hhdm_offset();
    println!("Got higher-half direct map offset");

    frame_alloc::init(regions, hhdm);
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
