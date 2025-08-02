use super::*;

fn return_cpu() -> CPU {
    CPU::default()
}

#[test]
fn test_new_cpu() {
    let cpu = return_cpu();
    assert_eq!(cpu.pc, 0);
    for i in 0..32 {
        assert_eq!(cpu.read_reg(i), 0);
    }
}

#[test]
fn test_read_write_regs() {
    let mut cpu = return_cpu();
    cpu.write_reg(1, 0xdead_beef);
    assert_eq!(cpu.read_reg(1), 0xdead_beef);
    cpu.write_reg(0, 0x12345678);
    assert_eq!(cpu.read_reg(0), 0);
}

#[test]
fn test_step_fetch_decode() {
    let mut cpu = return_cpu();
    cpu.memory.store_word(0, 0x0050_0113).unwrap();
    let insn = cpu.step().unwrap();
    assert_eq!(
        insn,
        Instruction::OP_IMM {
            rd: 2,
            rs1: 0,
            imm: 5
        }
    );
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_step_memory_error() {
    let mut cpu = CPU::new(4).unwrap();
    cpu.memory.store_word(0, 0x0000_0013).unwrap();

    let insn = cpu.step().unwrap();
    assert_eq!(
        insn,
        Instruction::OP_IMM {
            rd: 0,
            rs1: 0,
            imm: 0
        }
    );
    assert_eq!(cpu.pc, 4);

    let err = cpu.step().unwrap_err();
    assert!(matches!(err, CPUError::Memory(_)));
}
#[test]
fn test_step_decode_error() {
    let mut cpu = return_cpu();
    cpu.memory.store_byte(0, 0xFF).unwrap();
    let err = cpu.step().unwrap_err();
    matches!(err, CPUError::Decode(_));
}
