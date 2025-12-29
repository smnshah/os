#[derive(Clone, Copy)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    const PRESENT: u64 = 1 << 0;
    const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    pub fn new() -> Self {
        Self { entry: 0 }
    }

    pub fn is_present(&self) -> bool {
        (self.entry & Self::PRESENT) != 0
    }

    pub fn addr(&self) -> u64 {
        self.entry & Self::ADDR_MASK
    }

    pub fn set_addr(&mut self, addr: u64, flags: u64) {
        self.entry = addr | flags
    }

}

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}
