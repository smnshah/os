use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of;

pub const IDT_LEN: usize = 256;
static mut IDT: [IdtEntry; IDT_LEN] = [IdtEntry::empty(); IDT_LEN];

#[repr(C)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attrs: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn new(addr: u64) -> Self {
        Self { 
            offset_low: (addr & 0xFFFF) as u16,
            selector: 0x28,
            ist: 0,
            type_attrs: 0x8E,
            offset_mid: ((addr >> 16) & 0xFFFF) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0
        }    
    }

    pub const fn empty() -> Self {
        Self { 
            offset_low: 0, 
            selector: 0, 
            ist: 0, 
            type_attrs: 0, 
            offset_mid: 0, 
            offset_high: 0, 
            reserved: 0, 
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u64,
}

pub fn load_idt() {
    unsafe {
        let idtr = Idtr {
            limit: (size_of::<IdtEntry>() * IDT_LEN - 1) as u16,
            base: addr_of!(IDT) as u64,
        };

        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack));
    }
}
