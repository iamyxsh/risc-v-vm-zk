use super::*;

fn return_mem() -> Memory {
    Memory::new(128).unwrap()
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
    mem.store_byte(3, 0xAB).unwrap();
    assert_eq!(mem.load_byte(3).unwrap(), 0xAB);
}

#[test]
fn test_store_byte_out_of_bounds() {
    let mut mem = return_mem();
    let err = mem.store_byte(4, 0xFF).unwrap_err();
    assert_eq!(err, MemoryError::OutOfBounds { addr: 4, size: 4 });
}

#[test]
fn test_load_word_unaligned() {
    let mem = return_mem();
    let err = mem.load_word(2).unwrap_err();
    assert_eq!(err, MemoryError::UnalignedAccess { addr: 2, align: 4 });
}

#[test]
fn test_store_word_and_load_word() {
    let mut mem = return_mem();
    mem.store_word(4, 0x1234_5678).unwrap();
    assert_eq!(mem.load_word(4).unwrap(), 0x1234_5678);
}

#[test]
fn test_load_word_out_of_bounds() {
    let mem = return_mem();
    let err = mem.load_word(4).unwrap_err();
    assert_eq!(err, MemoryError::OutOfBounds { addr: 4, size: 4 });
}
