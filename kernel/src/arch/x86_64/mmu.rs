use core::arch::asm;

pub fn invalidate_page(virt_addr: u64) {
    unsafe {
        asm!(
            "invlpg [{}]",
            in(reg) virt_addr,
            options(nostack, preserves_flags)
        );
    }
}

pub fn read_cr2() -> u64 {
    let value: u64;
    unsafe {
        asm!(
            "mov {}, cr2",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

pub fn read_cr3() -> u64 {
    let value: u64;
    unsafe {
        asm!(
            "mov {}, cr3",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}
