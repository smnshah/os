#![no_std]
#![no_main]

mod arch;
mod boot;
mod io;
mod mm;

use core::panic::PanicInfo;

use crate::boot::limine;
use crate::mm::{frame, stack};
use crate::arch::x86_64::{cpu, idt, serial};

#[unsafe(no_mangle)]
extern "C" fn kernel_entry() -> ! {
    serial::init();
    println!("Initialized serial");

    let regions = limine::build_kernel_memory_map();
    println!("Built kernel memory map");

    let hhdm = limine::get_hhdm_offset();
    println!("Got higher-half direct map offset");

    frame::init(regions, hhdm);
    println!("Initialized frame allocator");

    let stack_top = stack::allocate_kernel_stack(hhdm)
        .expect("Kernel stack should be successfully allocated and mapped");
    println!("Allocated kernel stack");
    
    cpu::switch_stack(stack_top, kernel_main);
}

extern "C" fn kernel_main() -> ! {
    println!("Jumped to kernel stack");

    cpu::init();
    println!("Initialized cpu (gdt + tss)");

    idt::init();
    println!("Initialized idt");
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("KERNEL PANIC!");
    println!("{}", info);
    loop {}
}
