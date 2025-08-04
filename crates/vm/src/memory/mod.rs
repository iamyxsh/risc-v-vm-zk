pub mod errors;
pub mod tests;

use crate::constants::{DEFAULT_MEMORY_SIZE, MAX_MEMORY_SIZE};
use errors::MemoryError;

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Result<Self, MemoryError> {
        if size == 0 || size > MAX_MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds {
                addr: size as u32,
                size: MAX_MEMORY_SIZE,
            });
        }
        Ok(Memory {
            data: vec![0; size],
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn fetch(&self, addr: u32) -> Result<u32, MemoryError> {
        self.load_word(addr)
    }

    pub fn load_word(&self, addr: u32) -> Result<u32, MemoryError> {
        let addr = addr as usize;
        if addr + 4 > self.size {
            return Err(MemoryError::OutOfBounds {
                addr: addr as u32,
                size: self.size,
            });
        }
        if addr % 4 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: addr as u32,
                align: 4,
            });
        }
        let bytes = &self.data[addr..addr + 4];

        let word = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);
        Ok(word)
    }

    pub fn store_word(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        let addr = addr as usize;
        if addr + 4 > self.size {
            return Err(MemoryError::OutOfBounds {
                addr: addr as u32,
                size: self.size,
            });
        }
        if addr % 4 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: addr as u32,
                align: 4,
            });
        }
        let bytes = value.to_le_bytes();
        self.data[addr..addr + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn load_byte(&self, addr: u32) -> Result<u8, MemoryError> {
        let addr = addr as usize;
        if addr >= self.size {
            return Err(MemoryError::OutOfBounds {
                addr: addr as u32,
                size: self.size,
            });
        }
        Ok(self.data[addr])
    }

    pub fn store_byte(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        let addr = addr as usize;
        if addr >= self.size {
            return Err(MemoryError::OutOfBounds {
                addr: addr as u32,
                size: self.size,
            });
        }
        self.data[addr] = value;
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new(DEFAULT_MEMORY_SIZE).unwrap()
    }
}
