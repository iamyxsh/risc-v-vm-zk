use super::*;
use crate::{compiler::errors::ElfError, memory::Memory};
use goblin::elf::header::{ELFCLASS32, ELFDATA2LSB};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_empty_elf_no_segments() {
    let mut header = [0u8; 52];
    header[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    header[4] = ELFCLASS32;
    header[5] = ELFDATA2LSB;

    let entry: u32 = 0x1234;
    header[24..28].copy_from_slice(&entry.to_le_bytes());

    header[28..32].copy_from_slice(&52u32.to_le_bytes());
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&header).unwrap();

    let mut mem = Memory::new(1024).unwrap();
    let result = ElfLoader::load_elf(tmp.path(), &mut mem).unwrap();
    assert_eq!(result.entry, entry);
}

#[test]
fn test_load_single_pt_load() {
    let mut elf = Vec::new();
    let mut ehdr = [0u8; 52];
    ehdr[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    ehdr[4] = ELFCLASS32;
    ehdr[5] = ELFDATA2LSB;
    let entry = 0x200u32;
    ehdr[24..28].copy_from_slice(&entry.to_le_bytes());
    ehdr[28..32].copy_from_slice(&52u32.to_le_bytes());
    ehdr[42..44].copy_from_slice(&32u16.to_le_bytes());
    ehdr[44..46].copy_from_slice(&1u16.to_le_bytes());
    elf.extend_from_slice(&ehdr);
    let mut phdr = [0u8; 32];
    phdr[0..4].copy_from_slice(&1u32.to_le_bytes());
    phdr[4..8].copy_from_slice(&84u32.to_le_bytes());
    phdr[12..16].copy_from_slice(&10u32.to_le_bytes());
    phdr[16..20].copy_from_slice(&4u32.to_le_bytes());
    phdr[20..24].copy_from_slice(&8u32.to_le_bytes());
    elf.extend_from_slice(&phdr);
    elf.resize(84, 0);
    let data = [1u8, 2, 3, 4];
    elf.extend_from_slice(&data);

    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&elf).unwrap();

    let mut mem = Memory::new(128).unwrap();
    let res = ElfLoader::load_elf(tmp.path(), &mut mem).unwrap();
    assert_eq!(res.entry, entry);
    for i in 0..4u8 {
        assert_eq!(
            mem.load_byte((10u8 + i) as u32).unwrap(),
            *data.get(i as usize).unwrap()
        );
    }

    for i in 4..8 {
        assert_eq!(mem.load_byte(10 + i).unwrap(), 0);
    }
}

#[test]
fn test_unsupported_format() {
    let mut header = [0u8; 52];
    header[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    header[4] = 0;
    header[5] = 0;
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&header).unwrap();

    let mut mem = Memory::new(128).unwrap();
    let err = ElfLoader::load_elf(tmp.path(), &mut mem).unwrap_err();
    if let ElfError::UnsupportedFormat(c, d) = err {
        assert_eq!(c, 0);
        assert_eq!(d, 0);
    } else {
        panic!("expected UnsupportedFormat error");
    }
}

#[test]
fn test_segment_out_of_bounds() {
    let mut elf = Vec::new();
    let mut ehdr = [0u8; 52];
    ehdr[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    ehdr[4] = ELFCLASS32;
    ehdr[5] = ELFDATA2LSB;
    ehdr[28..32].copy_from_slice(&52u32.to_le_bytes());
    ehdr[42..44].copy_from_slice(&32u16.to_le_bytes());
    ehdr[44..46].copy_from_slice(&1u16.to_le_bytes());
    elf.extend_from_slice(&ehdr);
    let mut phdr = [0u8; 32];
    phdr[0..4].copy_from_slice(&1u32.to_le_bytes());
    phdr[4..8].copy_from_slice(&1000u32.to_le_bytes());
    phdr[12..16].copy_from_slice(&0u32.to_le_bytes());
    phdr[16..20].copy_from_slice(&10u32.to_le_bytes());
    phdr[20..24].copy_from_slice(&20u32.to_le_bytes());
    elf.extend_from_slice(&phdr);

    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&elf).unwrap();

    let mut mem = Memory::new(16).unwrap();
    let err = ElfLoader::load_elf(tmp.path(), &mut mem).unwrap_err();
    if let ElfError::SegmentOutOfBounds(paddr, sz) = err {
        assert_eq!(paddr, 0);
        assert_eq!(sz, 20);
    } else {
        panic!("expected SegmentOutOfBounds");
    }
}
