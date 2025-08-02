use super::*;

const DEFAULT_MEMORY_SIZE: usize = 128;
const DEFAULT_OOB_ADDR: usize = 129;
const DEFAULT_UNALIGNED_ADDR: usize = 2;

const DEFAULT_ADDR_TO_STORE: u32 = 4;
const DEFAULT_BYTE_TO_STORE: u8 = 0xAB;
const DEFAULT_WORD_TO_STORE: u32 = 0x1234_5678;

fn return_mem() -> Memory {
    Memory::new(DEFAULT_MEMORY_SIZE).unwrap()
}

#[test]
fn test_new_and_size() {
    let mem = return_mem();
    assert_eq!(mem.size(), 128);
}

#[test]
fn test_new_size_exceeds_max() {
    let too_big = MAX_MEMORY_SIZE + 1;
    let err = Memory::new(too_big).unwrap_err();
    assert_eq!(
        err,
        MemoryError::OutOfBounds {
            addr: too_big as u32,
            size: MAX_MEMORY_SIZE
        }
    );
}

#[test]
fn test_load_store_byte() {
    let mut mem = return_mem();
    mem.store_byte(DEFAULT_ADDR_TO_STORE, DEFAULT_BYTE_TO_STORE)
        .unwrap();
    assert_eq!(mem.load_byte(DEFAULT_ADDR_TO_STORE).unwrap(), 0xAB);
}

#[test]
fn test_store_byte_out_of_bounds() {
    let mut mem = return_mem();
    let err = mem
        .store_byte(DEFAULT_OOB_ADDR as u32, DEFAULT_BYTE_TO_STORE)
        .unwrap_err();
    assert_eq!(
        err,
        MemoryError::OutOfBounds {
            addr: DEFAULT_OOB_ADDR as u32,
            size: DEFAULT_MEMORY_SIZE
        }
    );
}

#[test]
fn test_load_word_unaligned() {
    let mem = return_mem();
    let err = mem.load_word(DEFAULT_UNALIGNED_ADDR as u32).unwrap_err();
    assert_eq!(
        err,
        MemoryError::UnalignedAccess {
            addr: DEFAULT_UNALIGNED_ADDR as u32,
            align: 4
        }
    );
}

#[test]
fn test_store_word_and_load_word() {
    let mut mem = return_mem();
    mem.store_word(DEFAULT_ADDR_TO_STORE, DEFAULT_WORD_TO_STORE)
        .unwrap();
    assert_eq!(mem.load_word(4).unwrap(), DEFAULT_WORD_TO_STORE);
}

#[test]
fn test_load_word_out_of_bounds() {
    let mem = return_mem();
    let err = mem.load_word(DEFAULT_OOB_ADDR as u32).unwrap_err();
    assert_eq!(
        err,
        MemoryError::OutOfBounds {
            addr: DEFAULT_OOB_ADDR as u32,
            size: DEFAULT_MEMORY_SIZE
        }
    );
}
