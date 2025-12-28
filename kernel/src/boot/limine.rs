use limine::BaseRevision;
use limine::request::{HhdmRequest, MemoryMapRequest};
use limine::{memory_map::Entry, memory_map::EntryType};
use crate::mm::types::{MemoryRegion, RegionType};

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

static mut MEMORY_REGIONS: [MemoryRegion; 64] = [MemoryRegion::empty(); 64];
static mut REGION_COUNT: usize = 0;

pub fn build_kernel_memory_map() -> &'static [MemoryRegion] {
    unsafe {
        let raw_entries = get_raw_entries();
        let mut idx = 0;

        for entry in raw_entries {
            let region = MemoryRegion {
                base: entry.base,
                length: entry.length,
                kind: get_entry_type(entry.entry_type),
            };

            MEMORY_REGIONS[idx] = region;
            idx += 1;
        }

        REGION_COUNT = idx;
        &MEMORY_REGIONS[0..REGION_COUNT]
    }
}

pub fn get_hhdm_offset() -> u64 {
    let response = HHDM_REQUEST
        .get_response()
        .expect("Bootloader should provide hhdm offset");

    response.offset()
}

fn get_raw_entries() -> &'static [&'static Entry] {
    let response = MEMORY_MAP_REQUEST
        .get_response()
        .expect("Bootloader should provide memory map");
    response.entries()
}

fn get_entry_type(entry_type: EntryType) -> RegionType {
    match entry_type {
        EntryType::USABLE => RegionType::Usable,
        EntryType::ACPI_RECLAIMABLE => RegionType::AcpiReclaimable,
        EntryType::BOOTLOADER_RECLAIMABLE => RegionType::Bootloader,
        EntryType::RESERVED
        | EntryType::ACPI_NVS
        | EntryType::BAD_MEMORY
        | EntryType::EXECUTABLE_AND_MODULES
        | EntryType::FRAMEBUFFER => RegionType::Reserved,
        _ => RegionType::Unknown,
    }
}
