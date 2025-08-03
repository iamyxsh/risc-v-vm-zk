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

#[test]
fn test_add_and_sub() {
    let mut cpu = return_cpu();
    cpu.write_reg(1, 10);
    cpu.write_reg(2, 3);
    cpu.execute(Instruction::OP {
        rd: 3,
        rs1: 1,
        rs2: 2,
        funct7: 0x00,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(3), 13);
    cpu.execute(Instruction::OP {
        rd: 4,
        rs1: 1,
        rs2: 2,
        funct7: 0x20,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(4), 7);
}

#[test]
fn test_load_store() {
    let mut cpu = return_cpu();
    cpu.write_reg(1, 100);
    cpu.write_reg(2, 0xDEADBEEF);
    cpu.execute(Instruction::STORE {
        rs1: 1,
        rs2: 2,
        imm: 0,
    })
    .unwrap();
    cpu.execute(Instruction::LOAD {
        rd: 3,
        rs1: 1,
        imm: 0,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(3), 0xDEADBEEF);
}

#[test]
fn test_mul_div_rem() {
    let mut cpu = return_cpu();
    cpu.write_reg(1, 15);
    cpu.write_reg(2, 4);
    cpu.execute(Instruction::MUL {
        rd: 3,
        rs1: 1,
        rs2: 2,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(3), 60);
    cpu.execute(Instruction::DIV {
        rd: 4,
        rs1: 1,
        rs2: 2,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(4), 3);
    cpu.execute(Instruction::DIVU {
        rd: 5,
        rs1: 1,
        rs2: 2,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(5), 3);
    cpu.execute(Instruction::REM {
        rd: 6,
        rs1: 1,
        rs2: 2,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(6), 3);
    cpu.execute(Instruction::REMU {
        rd: 7,
        rs1: 1,
        rs2: 2,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(7), 3);
}

#[test]
fn test_auipc_jal_jalr() {
    let mut cpu = return_cpu();

    cpu.pc = 100;
    cpu.execute(Instruction::AUIPC { rd: 1, imm: 20 }).unwrap();
    assert_eq!(cpu.read_reg(1), 120);

    cpu.pc = 200;
    cpu.execute(Instruction::JAL { rd: 2, imm: 16 }).unwrap();
    assert_eq!(cpu.read_reg(2), 204);
    assert_eq!(cpu.pc, 216);

    cpu.write_reg(3, 0);
    cpu.execute(Instruction::JALR {
        rd: 3,
        rs1: 1,
        imm: 4,
    })
    .unwrap();

    assert_eq!(cpu.read_reg(3), 220);
    assert_eq!(cpu.pc, 124);
}

#[test]
fn test_shift_imm() {
    let mut cpu = return_cpu();
    cpu.write_reg(1, 1);
    cpu.execute(Instruction::SHIFT_IMM {
        rd: 2,
        rs1: 1,
        shamt: 3,
        is_srai: false,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(2), 8);
    cpu.write_reg(1, u32::MAX);
    cpu.execute(Instruction::SHIFT_IMM {
        rd: 2,
        rs1: 1,
        shamt: 1,
        is_srai: true,
    })
    .unwrap();
    assert_eq!(cpu.read_reg(2), u32::MAX >> 1);
}

#[test]
fn test_branch_equal() {
    let mut cpu = return_cpu();
    cpu.pc = 10;
    cpu.write_reg(1, 5);
    cpu.write_reg(2, 5);
    cpu.execute(Instruction::BRANCH {
        rs1: 1,
        rs2: 2,
        imm: 8,
    })
    .unwrap();
    assert_eq!(cpu.pc, 18);
    cpu.pc = 10;
    cpu.write_reg(2, 6);
    cpu.execute(Instruction::BRANCH {
        rs1: 1,
        rs2: 2,
        imm: 8,
    })
    .unwrap();
    assert_eq!(cpu.pc, 10);
}

#[test]
fn test_ecall_ebreak() {
    let mut cpu = return_cpu();
    cpu.pc = 30;
    cpu.execute(Instruction::ECALL).unwrap();
    assert_eq!(cpu.pc, 30);
    cpu.execute(Instruction::EBREAK).unwrap();
    assert_eq!(cpu.pc, 30);
}
