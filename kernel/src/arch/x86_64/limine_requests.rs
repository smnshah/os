use limine::request::{HhdmRequest, MemoryMapRequest};

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

