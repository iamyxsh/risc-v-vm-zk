mod errors;
mod tests;

use crate::itxs::Instruction;
use errors::DecodeError;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = (word & 0x7F) as u8;
    match opcode {
        0x37 => decode_u_type(word, |rd, imm| Instruction::LUI { rd, imm }),
        0x17 => decode_u_type(word, |rd, imm| Instruction::AUIPC { rd, imm }),
        0x6F => decode_j_type(word, |rd, imm| Instruction::JAL { rd, imm }),
        0x67 => decode_i_type(word, |rd, rs1, imm| Instruction::JALR { rd, rs1, imm }),
        0x03 => decode_load_type(word),
        0x13 => {
            let f3 = funct3(word);
            if f3 == 0x1 || f3 == 0x5 {
                decode_shift_imm_type(word)
            } else {
                decode_op_imm_type(word)
            }
        }
        0x23 => decode_store_type(word),
        0x63 => decode_branch_type(word),
        0x33 => decode_op_type(word),
        0x73 => decode_system(word),
        0x3B => decode_op_type(word),
        _ => Err(DecodeError::UnknownOpcode(opcode)),
    }
}

fn rd(word: u32) -> u8 {
    ((word >> 7) & 0x1F) as u8
}
fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0x07) as u8
}
fn rs1(word: u32) -> u8 {
    ((word >> 15) & 0x1F) as u8
}
fn rs2(word: u32) -> u8 {
    ((word >> 20) & 0x1F) as u8
}
fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0x7F) as u8
}

fn immediate_i(word: u32) -> i32 {
    (word as i32) >> 20
}
fn immediate_s(word: u32) -> i32 {
    let lo = (word >> 7) & 0x1F;
    let hi = (word >> 25) & 0x7F;
    let raw = (hi << 5) | lo;
    ((raw as i32) << 20) >> 20
}
fn immediate_b(word: u32) -> i32 {
    let b11_5 = (word >> 25) & 0x3F;
    let b4_1 = (word >> 8) & 0x0F;
    let b11 = (word >> 7) & 0x01;
    let b12 = (word >> 31) & 0x01;
    let raw = (b12 << 12) | (b11 << 11) | (b11_5 << 5) | (b4_1 << 1);
    ((raw as i32) << 19) >> 19
}
fn immediate_u(word: u32) -> u32 {
    word & 0xFFFFF000
}
fn immediate_j(word: u32) -> i32 {
    let b20 = (word >> 31) & 0x01;
    let b10_1 = (word >> 21) & 0x3FF;
    let b11 = (word >> 20) & 0x01;
    let b19_12 = (word >> 12) & 0xFF;
    let raw = (b20 << 20) | (b19_12 << 12) | (b11 << 11) | (b10_1 << 1);
    ((raw as i32) << 11) >> 11
}

fn decode_u_type<F>(word: u32, ctor: F) -> Result<Instruction, DecodeError>
where
    F: Fn(u8, u32) -> Instruction,
{
    Ok(ctor(rd(word), immediate_u(word)))
}
fn decode_i_type<F>(word: u32, ctor: F) -> Result<Instruction, DecodeError>
where
    F: Fn(u8, u8, i32) -> Instruction,
{
    Ok(ctor(rd(word), rs1(word), immediate_i(word)))
}
fn decode_j_type<F>(word: u32, ctor: F) -> Result<Instruction, DecodeError>
where
    F: Fn(u8, i32) -> Instruction,
{
    Ok(ctor(rd(word), immediate_j(word)))
}
fn decode_load_type(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::LOAD {
        rd: rd(word),
        rs1: rs1(word),
        imm: immediate_i(word),
    })
}
fn decode_op_imm_type(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::OP_IMM {
        rd: rd(word),
        rs1: rs1(word),
        imm: immediate_i(word),
    })
}
fn decode_shift_imm_type(word: u32) -> Result<Instruction, DecodeError> {
    let shamt = ((word >> 20) & 0x1F) as u8;
    let is_srai = funct7(word) == 0x20;
    Ok(Instruction::SHIFT_IMM {
        rd: rd(word),
        rs1: rs1(word),
        shamt,
        is_srai,
    })
}
fn decode_store_type(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::STORE {
        rs1: rs1(word),
        rs2: rs2(word),
        imm: immediate_s(word),
    })
}
fn decode_branch_type(word: u32) -> Result<Instruction, DecodeError> {
    Ok(Instruction::BRANCH {
        rs1: rs1(word),
        rs2: rs2(word),
        imm: immediate_b(word),
    })
}
fn decode_op_type(word: u32) -> Result<Instruction, DecodeError> {
    let f3 = funct3(word);
    let f7 = funct7(word);
    let rd = rd(word);
    let rs1 = rs1(word);
    let rs2 = rs2(word);
    match (f3, f7) {
        (0x0, 0x00) => Ok(Instruction::OP {
            rd,
            rs1,
            rs2,
            funct7: f7,
        }),
        (0x0, 0x20) => Ok(Instruction::OP {
            rd,
            rs1,
            rs2,
            funct7: f7,
        }),

        (0x0, 0x01) => Ok(Instruction::MUL { rd, rs1, rs2 }),
        (0x4, 0x01) => Ok(Instruction::DIV { rd, rs1, rs2 }),
        (0x5, 0x01) => Ok(Instruction::DIVU { rd, rs1, rs2 }),
        (0x6, 0x01) => Ok(Instruction::REM { rd, rs1, rs2 }),
        (0x7, 0x01) => Ok(Instruction::REMU { rd, rs1, rs2 }),
        _ => Err(DecodeError::InvalidInstruction(word)),
    }
}
fn decode_system(word: u32) -> Result<Instruction, DecodeError> {
    match (funct3(word), immediate_i(word)) {
        (0x0, 0x0) => Ok(Instruction::ECALL),
        (0x0, 0x1) => Ok(Instruction::EBREAK),
        _ => Err(DecodeError::InvalidInstruction(word)),
    }
}
