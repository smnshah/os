use core::arch::asm;
use core::mem::size_of;

pub const TSS_LIMIT: u16 = size_of::<Tss>() as u16 - 1;

#[repr(C, packed)]
pub struct Tss {
    _reserved1: u32,
    rsp0: u64,
    _rsp1: u64,
    _rsp2: u64,
    _reserved2: u64,
    ist1: u64,
    _ist2: u64,
    _ist3: u64,
    _ist4: u64,
    _ist5: u64,
    _ist6: u64,
    _ist7: u64,
    _reserved3: u64,   
    _reserved4: u16,
    iopb_offset: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Self {
            _reserved1: 0,
            rsp0: 0,
            _rsp1: 0,
            _rsp2: 0,
            _reserved2: 0,
            ist1: 0,
            _ist2: 0,
            _ist3: 0,
            _ist4: 0,
            _ist5: 0,
            _ist6: 0,
            _ist7: 0,
            _reserved3: 0,
            _reserved4: 0,
            iopb_offset: size_of::<Tss>() as u16,
        } 
    }

    pub unsafe fn load(selector: u16) {
        unsafe {
            asm!("ltr {0:x}", in(reg) selector, options(nostack));
        }
    }

    pub fn init(&mut self, df_stack_top: u64) {
        self.ist1 = df_stack_top;
    }
}


