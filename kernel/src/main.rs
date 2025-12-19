#![no_std]
#![no_main]

mod arch;

use crate::arch::x86_64::mem::{frame_alloc, memory_map};
use crate::arch::x86_64::serial;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    serial::init();
    serial::write_str(b"Hello world!\n");

    let regions = memory_map::build_kernel_memory_map();
    let hhdm = memory_map::get_hhdm_offset();
    frame_alloc::init(regions, hhdm);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
