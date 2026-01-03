use crate::arch::x86_64::mmu;

const HUGE_1G_MASK: u64 = 0x3FFF_FFFF;
const HUGE_2M_MASK: u64 = 0x1F_FFFF;
const PAGE_4K_MASK: u64 = 0xFFF;

#[derive(Clone, Copy)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    const PRESENT: u64 = 1 << 0;
    const HUGE: u64 = 1 << 7;
    const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    pub fn is_present(&self) -> bool {
        (self.entry & Self::PRESENT) != 0
    }

    pub fn is_huge(&self) -> bool {
        (self.entry & Self::HUGE) != 0
    }

    pub fn addr(&self) -> u64 {
        self.entry & Self::ADDR_MASK
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    fn get_entry(&self, virt_addr: u64, bit_shift: u32) -> PageTableEntry {
        let idx = ((virt_addr >> bit_shift) & 0x1FF) as usize;
        self.entries[idx]
    }
}

unsafe fn table_at(phys_addr: u64, hhdm_offset: u64) -> &'static PageTable {
    unsafe { &*((phys_addr + hhdm_offset) as *const PageTable) }
}

pub fn translate(virt_addr: u64, hhdm_offset: u64) -> Option<u64> {
    let pml4 = unsafe { table_at(mmu::read_cr3() & PageTableEntry::ADDR_MASK, hhdm_offset) };
    let pml4_entry = pml4.get_entry(virt_addr, 39);
    if !pml4_entry.is_present() { return None; } 

    let pdp = unsafe { table_at(pml4_entry.addr(), hhdm_offset) };
    let pdp_entry = pdp.get_entry(virt_addr, 30);
    if !pdp_entry.is_present() { return None; }

    if pdp_entry.is_huge() { 
        return Some(pdp_entry.addr() | (virt_addr & HUGE_1G_MASK)); 
    }

    let pd = unsafe { table_at(pdp_entry.addr(), hhdm_offset) };
    let pd_entry = pd.get_entry(virt_addr, 21);
    if !pd_entry.is_present() { return None; }

    if pd_entry.is_huge() {
        return Some(pd_entry.addr() | (virt_addr & HUGE_2M_MASK));
    }

    let pt = unsafe { table_at(pd_entry.addr(), hhdm_offset) };
    let pt_entry = pt.get_entry(virt_addr, 12);
    if !pt_entry.is_present() { return None; }

    Some(pt_entry.addr() | (virt_addr & PAGE_4K_MASK))
}
