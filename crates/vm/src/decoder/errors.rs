#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnknownOpcode(u8),
    InvalidInstruction(u32),
}
