use crate::arch::x86_64::mem::memory_map::{MemoryRegion, RegionType};

const PAGE_SIZE: usize = 4096;

static mut BITMAP_ADDR: *mut u8 = core::ptr::null_mut();
static mut BITMAP_PHYS_START: u64 = 0;
static mut BITMAP_SIZE: usize = 0;
static mut MAX_PFN: usize = 0;

pub fn init(regions: &[MemoryRegion], hhdm_offset: u64) {
    unsafe {
        MAX_PFN = max_pfn(regions);
        BITMAP_SIZE = (MAX_PFN + 7) / 8;

        let first_usable = first_usable_region(regions);
        assert!(BITMAP_SIZE < first_usable.length as usize);

        BITMAP_PHYS_START = first_usable.base;
        let bitmap_vaddr = BITMAP_PHYS_START + hhdm_offset;
        BITMAP_ADDR = bitmap_vaddr as *mut u8;

        for i in 0..BITMAP_SIZE {
            BITMAP_ADDR.add(i).write(0xff);
        }
        
        for region in regions {
            if matches!(region.kind, RegionType::Usable) {
                let start = start_frame(region);
                let end = end_frame(region);
                for frame in start..=end {
                    if frame < MAX_PFN && !bitmap_overlaps(frame) {
                        mark_frame_free(frame);
                    }
                }
            }
        }
    }
}

pub fn alloc() -> Option<u64> {
    unsafe {
        for pfn in 0..MAX_PFN {
            if is_frame_free(pfn) {
                mark_frame_allocated(pfn);
                return Some((pfn * PAGE_SIZE) as u64);
            }
        }
    }
    None
}

pub fn free(frame_addr: u64) {
    let pfn = (frame_addr as usize) / PAGE_SIZE;
    unsafe {
        if pfn >= MAX_PFN {
            return;
        }
    }
    mark_frame_free(pfn);
}

fn bitmap_overlaps(pfn: usize) -> bool {
    unsafe {
        let start = BITMAP_PHYS_START as usize / PAGE_SIZE;
        let end = (BITMAP_PHYS_START as usize + BITMAP_SIZE - 1) / PAGE_SIZE;
        pfn >= start && pfn <= end
    }
}

fn first_usable_region(regions: &[MemoryRegion]) -> &MemoryRegion {
    regions
        .iter()
        .find(|r| matches!(r.kind, RegionType::Usable))
        .expect("Bootloader should provide at least one usable region")
}

fn frame_to_byte_bit(pfn: usize) -> (usize, usize) {
    let byte_idx = pfn / 8;
    let bit_idx = pfn % 8;
    (byte_idx, bit_idx)
}

fn is_frame_free(pfn: usize) -> bool {
    let (byte_idx, bit_idx) = frame_to_byte_bit(pfn);
    unsafe {
        if byte_idx >= BITMAP_SIZE {
            return false
        }
        let byte = BITMAP_ADDR.add(byte_idx).read();
        ((byte >> bit_idx) & 1) == 0        
    }
}

fn mark_frame_free(pfn: usize) {
    let (byte_idx, bit_idx) = frame_to_byte_bit(pfn);
    unsafe {
        if byte_idx >= BITMAP_SIZE {
            return;
        }
            
        let byte = BITMAP_ADDR.add(byte_idx).read();
        let new_byte = byte & !(1 << bit_idx);
        BITMAP_ADDR.add(byte_idx).write(new_byte);
    }
}

fn mark_frame_allocated(pfn: usize) {
    let (byte_idx, bit_idx) = frame_to_byte_bit(pfn);
    unsafe {
        if byte_idx >= BITMAP_SIZE {
            return;
        }
            
        let byte = BITMAP_ADDR.add(byte_idx).read();
        let new_byte = byte | (1 << bit_idx);
        BITMAP_ADDR.add(byte_idx).write(new_byte);
    }
}

fn max_pfn(regions: &[MemoryRegion]) -> usize {
    let mut max = 0;
    for region in regions {
        let end_addr = region.base + region.length - 1;
        let end_pfn = (end_addr as usize) / PAGE_SIZE;
        if end_pfn > max {
            max = end_pfn
        }
    }
    max + 1
}

fn start_frame(region: &MemoryRegion) -> usize {
    (region.base as usize) / PAGE_SIZE
}

fn end_frame(region: &MemoryRegion) -> usize {
    ((region.base + region.length - 1) as usize) / PAGE_SIZE
}
