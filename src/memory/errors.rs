#[derive(Debug, PartialEq)]
pub enum MemoryError {
    OutOfBounds { addr: u32, size: usize },
    UnalignedAccess { addr: u32, align: u32 },
}
