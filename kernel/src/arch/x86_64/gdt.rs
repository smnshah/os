use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of;

pub const GDT_LEN: usize = 3;
pub const KERNEL_CODE_IDX: usize = 1;
pub const KERNEL_DATA_IDX: usize = 2;
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;

static mut GDT: Gdt = Gdt::new();

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_high_and_flags: u8,
    base_high: u8,

}

impl GdtEntry {
    const fn empty() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_high_and_flags: 0,
            base_high: 0,
        }
    }

    const fn kernel_code() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0x9a,
            limit_high_and_flags: 0xa0, 
            base_high: 0,
        }
    }

    const fn kernel_data() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0x92,
            limit_high_and_flags: 0,
            base_high: 0,
        }
    }
}

#[repr(C)]
pub struct Gdt {
    entries: [GdtEntry; GDT_LEN],
}

impl Gdt {
    const fn new() -> Self {
        Self {
            entries: [GdtEntry::empty(); GDT_LEN],
        }
    }

    pub fn set_entry(&mut self, idx: usize, entry: GdtEntry) {
        self.entries[idx] = entry;
    }

    fn load(&self) {
        unsafe {
            let descriptor = GdtDescriptor {
                size: (size_of::<GdtEntry>() * GDT_LEN - 1) as u16,
                offset: addr_of!(self.entries) as u64, 
            };

            asm!("lgdt [{}]", in(reg) &descriptor, options(readonly, nostack));
        }
    }
}

pub fn init() {
    unsafe {
        let gdt = &raw mut GDT;
        (*gdt).set_entry(KERNEL_CODE_IDX, GdtEntry::kernel_code());
        (*gdt).set_entry(KERNEL_DATA_IDX, GdtEntry::kernel_data());
        (*gdt).load();
        reload_segments();
    }
}

unsafe fn reload_segments() {
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
