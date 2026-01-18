use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of;
use super::tss::TSS_LIMIT;

pub const GDT_LEN: usize = 3;
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;
pub const TSS_SELECTOR: u16 = 0x18;

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C, packed)]
struct TssDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_high_flags: u8,
    base_high: u8,
    base_upper: u32,
    _reserved: u32,
}

impl TssDescriptor {
    pub const fn empty() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_high_flags: 0,
            base_high: 0,
            base_upper: 0,
            _reserved: 0,
        }
    }

    pub fn init(&mut self, tss_addr: u64) {
        self.limit_low = (TSS_LIMIT) & 0xFFFF;
        self.limit_high_flags = 0;

        self.base_low = ((tss_addr) & 0xFFFF) as u16;
        self.base_mid = ((tss_addr >> 16) & 0xFF) as u8;
        self.base_high = ((tss_addr >> 24) & 0xFF) as u8;
        self.base_upper = (tss_addr >> 32) as u32;

        self.access = 0x89; // Present | Available 64-bit TSS
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_high_flags: u8,
    base_high: u8,

}

impl GdtEntry {
    const fn empty() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_high_flags: 0,
            base_high: 0,
        }
    }

    const fn kernel_code() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0x9a,
            limit_high_flags: 0xa0, 
            base_high: 0,
        }
    }

    const fn kernel_data() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0x92,
            limit_high_flags: 0,
            base_high: 0,
        }
    }
}

#[repr(C)]
pub struct Gdt {
    entries: [GdtEntry; GDT_LEN],
    tss_desc: TssDescriptor, 
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            entries: [
                GdtEntry::empty(),
                GdtEntry::kernel_code(),
                GdtEntry::kernel_data(),
            ],
            tss_desc: TssDescriptor::empty(),
        }
    }

    fn load_gdt(&self) {
        unsafe {
            let descriptor = GdtDescriptor {
                size: (size_of::<Gdt>() - 1) as u16,
                offset: addr_of!(self.entries) as u64, 
            };

            asm!("lgdt [{}]", in(reg) &descriptor, options(readonly, nostack));
        }
    }

    pub fn init_tss(&mut self, tss_addr: u64) {
        self.tss_desc.init(tss_addr);
    }

    pub unsafe fn load(&self) {
        self.load_gdt();
        reload_segments();
    }
}

fn reload_segments() {
    unsafe {
        asm!(
            "mov ax, {data_sel}",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",

            "push {code_sel}",      
            "lea rax, [rip + 2f]",  
            "push rax",             
            "retfq",                
            "2:",                  

            code_sel = const KERNEL_CODE_SELECTOR,
            data_sel = const KERNEL_DATA_SELECTOR,
            out("rax") _,
            options(preserves_flags)
        );
    }
}
