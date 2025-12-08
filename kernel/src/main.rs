#![no_std]
#![no_main]

mod arch;

use crate::arch::x86_64::{memory_map, serial};
use core::panic::PanicInfo;

#[unsafe(no_mangle)] 
extern "C" fn kernel_main() -> ! {

    serial::init();
    serial::write_str(b"Hello world!\n");

    memory_map::build_kernel_memory_map();
    unsafe {
        memory_map::HHDM_OFFSET = memory_map::get_hhdm_offset();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}