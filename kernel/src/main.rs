#![no_std]
#![no_main]

mod arch;

use crate::arch::x86_64::serial;
use core::panic::PanicInfo;

#[unsafe(no_mangle)] 
unsafe extern "C" fn kernel_main() -> ! {

    serial::init();
    serial::write_str(b"Hello world!\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}