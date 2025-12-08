use limine::request::{MemoryMapRequest, HhdmRequest};

#[used]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
#[used]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();