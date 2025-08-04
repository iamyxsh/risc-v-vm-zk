use std::{fs::File, io::Read, path::Path};

use goblin::elf::{Elf, program_header::PT_LOAD};

use crate::memory::Memory;

mod errors;
mod tests;

#[derive(Debug)]
pub struct LoadResult {
    pub entry: u32,
}

pub struct ElfLoader;

impl ElfLoader {
    pub fn load_elf<P: AsRef<Path>>(
        path: P,
        memory: &mut Memory,
    ) -> Result<LoadResult, errors::ElfError> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        use goblin::elf::header::{EI_CLASS, EI_DATA, ELFCLASS32, ELFDATA2LSB};
        if buffer.len() > EI_DATA as usize {
            let class = buffer[EI_CLASS as usize];
            let data = buffer[EI_DATA as usize];
            if class != ELFCLASS32 || data != ELFDATA2LSB {
                return Err(errors::ElfError::UnsupportedFormat(class, data));
            }
        }

        let elf = Elf::parse(&buffer)?;

        for ph in &elf.program_headers {
            if ph.p_type != PT_LOAD {
                continue;
            }

            let paddr = ph.p_paddr as u32;
            let filesz = ph.p_filesz as usize;
            let memsz = ph.p_memsz as usize;

            if (paddr as usize).saturating_add(memsz) > memory.size() {
                return Err(errors::ElfError::SegmentOutOfBounds(paddr, memsz));
            }

            let base_offset = ph.p_offset as usize;
            for i in 0..filesz {
                let byte = buffer[base_offset + i];
                memory
                    .store_byte(paddr + i as u32, byte)
                    .expect("bounds check ensures safety");
            }

            for i in filesz..memsz {
                memory
                    .store_byte(paddr + i as u32, 0)
                    .expect("bounds check ensures safety");
            }
        }

        Ok(LoadResult {
            entry: elf.header.e_entry as u32,
        })
    }
}
