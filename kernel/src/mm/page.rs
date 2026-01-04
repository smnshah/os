use core::ptr::write_bytes;

use crate::arch::x86_64::mmu;
use crate::mm::frame;

#[derive(Clone, Copy)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    const PRESENT: u64 = 1 << 0;
    const WRITABLE: u64 = 1 << 1;
    const HUGE: u64 = 1 << 7;
    const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;
    
    pub fn new(value: u64) -> Self {
        Self { entry: value }
    }

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
    fn get_entry(&mut self, virt_addr: u64, bit_shift: u32) -> &mut PageTableEntry {
        let idx = ((virt_addr >> bit_shift) & 0x1FF) as usize;
        &mut self.entries[idx]
    }

    fn set_entry(&mut self, virt_addr: u64, bit_shift: u32, entry: PageTableEntry) {
        let idx = ((virt_addr >> bit_shift) & 0x1FF) as usize;
        self.entries[idx] = entry
    }
}

pub enum PteError {
    HugePage,
    NotMapped,
    OutOfMemory,
}

pub enum MapError {
    AlreadyMapped,
    HugePage,
    OutOfMemory,
}

pub fn map() {

}

pub fn unmap() {

}

unsafe fn table_at(phys_addr: u64, hhdm_offset: u64) -> &'static mut PageTable {
    unsafe { &mut *((phys_addr + hhdm_offset) as *mut PageTable) }
}

fn get_pte_mut(virt_addr: u64, hhdm_offset: u64, allocate: bool) -> Result<&'static mut PageTableEntry, PteError> {
    let pml4 = unsafe { table_at(mmu::read_cr3() & PageTableEntry::ADDR_MASK, hhdm_offset) };
    
    let pdp = get_or_allocate_table(pml4, virt_addr, 39, hhdm_offset, allocate)?;
    if pdp.get_entry(virt_addr, 30).is_huge() { return Err(PteError::HugePage); } 
    
    let pd = get_or_allocate_table(pdp, virt_addr, 30, hhdm_offset, allocate)?;
    if pd.get_entry(virt_addr, 21).is_huge() { return Err(PteError::HugePage); }

    let pt = get_or_allocate_table(pd, virt_addr, 21, hhdm_offset, allocate)?;
    Ok(pt.get_entry(virt_addr, 12))
}

fn get_or_allocate_table(page_table: &mut PageTable, virt_addr: u64, bit_shift: u32, hhdm_offset: u64, allocate: bool) -> Result<&'static mut PageTable, PteError> {
    if !page_table.get_entry(virt_addr, bit_shift).is_present() { 
        if !allocate { return Err(PteError::NotMapped); }
        let new_table_phys = frame::alloc().ok_or(PteError::OutOfMemory)?;
        unsafe { write_bytes((new_table_phys + hhdm_offset) as *mut u8, 0x00, 4096); }
        page_table.set_entry(virt_addr, bit_shift, PageTableEntry::new(new_table_phys | PageTableEntry::PRESENT | PageTableEntry::WRITABLE));
    }
    Ok(unsafe { table_at(page_table.get_entry(virt_addr, bit_shift).addr(), hhdm_offset) })
}
