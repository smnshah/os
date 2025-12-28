#[derive(Clone, Copy, Debug)]
pub enum RegionType {
    Usable,
    Reserved,
    AcpiReclaimable,
    Bootloader,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub base: u64,
    pub length: u64,
    pub kind: RegionType,
}

impl MemoryRegion {
    pub const fn empty() -> Self {
        Self {
            base: 0,
            length: 0,
            kind: RegionType::Unknown,
        }
    }
}
