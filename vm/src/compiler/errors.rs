use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElfError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ELF parse error: {0}")]
    Parse(#[from] goblin::error::Error),

    #[error("unsupported ELF class {0} / encoding {1}")]
    UnsupportedFormat(u8, u8),

    #[error("segment at paddr=0x{0:x} (memsz={1}) out of bounds")]
    SegmentOutOfBounds(u32, usize),
}
