use super::*;

#[test]
fn decode_lui() {
    let code = 0x123450B7;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::LUI {
            rd: 1,
            imm: 0x12345000
        }
    );
}

#[test]
fn decode_auipc() {
    let code = 0x0000F097;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::AUIPC {
            rd: 1,
            imm: 0x0000F000
        }
    );
}

#[test]
fn decode_add() {
    let code = 0x007302B3;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::OP {
            rd: 5,
            rs1: 6,
            rs2: 7,
            funct7: 0x00
        }
    );
}

#[test]
fn decode_sub() {
    let code = 0x407302B3;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::OP {
            rd: 5,
            rs1: 6,
            rs2: 7,
            funct7: 0x20
        }
    );
}

#[test]
fn decode_mul_div_rem() {
    assert_eq!(
        decode(0x023100B3).unwrap(),
        Instruction::MUL {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
    assert_eq!(
        decode(0x023140B3).unwrap(),
        Instruction::DIV {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
    assert_eq!(
        decode(0x023150B3).unwrap(),
        Instruction::DIVU {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
    assert_eq!(
        decode(0x023160B3).unwrap(),
        Instruction::REM {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
    assert_eq!(
        decode(0x023170B3).unwrap(),
        Instruction::REMU {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
}

#[test]
fn decode_addi() {
    // ADDI x1, x2, -5
    let code = 0xFFB10093;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::OP_IMM {
            rd: 1,
            rs1: 2,
            imm: -5
        }
    );
}

#[test]
fn decode_load_store() {
    let lw = 0x00412083;
    assert_eq!(
        decode(lw).unwrap(),
        Instruction::LOAD {
            rd: 1,
            rs1: 2,
            imm: 4
        }
    );
    let sw = 0x0030A423;
    assert_eq!(
        decode(sw).unwrap(),
        Instruction::STORE {
            rs1: 1,
            rs2: 3,
            imm: 8
        }
    );
}

#[test]
fn decode_branch() {
    // BEQ x1, x2, 16
    let code = 0x00208863;
    assert_eq!(
        decode(code).unwrap(),
        Instruction::BRANCH {
            rs1: 1,
            rs2: 2,
            imm: 16
        }
    );
}

#[test]
fn decode_jal_jalr() {
    let jal = 0x000000EF;
    assert_eq!(decode(jal).unwrap(), Instruction::JAL { rd: 1, imm: 0 });
    let jalr = 0x000100E7;
    assert_eq!(
        decode(jalr).unwrap(),
        Instruction::JALR {
            rd: 1,
            rs1: 2,
            imm: 0
        }
    );
}

#[test]
fn decode_shift_imm() {
    let slli = 0x00311093;
    assert_eq!(
        decode(slli).unwrap(),
        Instruction::SHIFT_IMM {
            rd: 1,
            rs1: 2,
            shamt: 3,
            is_srai: false
        }
    );
    let srai = 0x40311093;
    assert_eq!(
        decode(srai).unwrap(),
        Instruction::SHIFT_IMM {
            rd: 1,
            rs1: 2,
            shamt: 3,
            is_srai: true
        }
    );
}

#[test]
fn decode_system_calls() {
    assert_eq!(decode(0x00000073).unwrap(), Instruction::ECALL);
    assert_eq!(decode(0x00100073).unwrap(), Instruction::EBREAK);
}

#[test]
fn decode_unknown_and_invalid() {
    assert_eq!(decode(0), Err(DecodeError::UnknownOpcode(0)));
    let bad_sys = 0x00200073;
    assert_eq!(
        decode(bad_sys),
        Err(DecodeError::InvalidInstruction(bad_sys))
    );
}
