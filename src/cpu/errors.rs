use crate::decoder::errors::DecodeError;
use crate::memory::errors::MemoryError;

#[derive(Debug, PartialEq)]
pub enum CPUError {
    Memory(MemoryError),
    Decode(DecodeError),
}

impl From<MemoryError> for CPUError {
    fn from(e: MemoryError) -> Self {
        CPUError::Memory(e)
    }
}
impl From<DecodeError> for CPUError {
    fn from(e: DecodeError) -> Self {
        CPUError::Decode(e)
    }
}
