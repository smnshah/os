use super::{frame, page};
use super::page::PageTableEntry;

const KERNEL_STACK_BASE: u64 = 0xffffffff90000000;
const KERNEL_STACK_PAGES: usize = 4;
const PAGE_SIZE: u64 = 4096;

#[derive(Debug)]
pub enum StackError {
    OutofFrames,
    MapFailed,
}

pub fn allocate_kernel_stack(hhdm_offset: u64) -> Result<u64, StackError> {
    for i in 1..=KERNEL_STACK_PAGES {
        let virt_addr = KERNEL_STACK_BASE + (i as u64 * PAGE_SIZE);
        let phys_addr = frame::alloc().ok_or(StackError::OutofFrames)?;
        page::map(virt_addr, phys_addr, PageTableEntry::PRESENT | PageTableEntry::WRITABLE, hhdm_offset)
            .map_err(|_| StackError::MapFailed)?;
    }

    page::map_guard(KERNEL_STACK_BASE, hhdm_offset).map_err(|_| StackError::MapFailed)?;

    Ok(KERNEL_STACK_BASE + ((KERNEL_STACK_PAGES + 1) as u64 * PAGE_SIZE))
}
