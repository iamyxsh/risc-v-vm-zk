#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    LUI {
        rd: u8,
        imm: u32,
    },
    AUIPC {
        rd: u8,
        imm: u32,
    },
    JAL {
        rd: u8,
        imm: i32,
    },
    JALR {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    LOAD {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    OP_IMM {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    SHIFT_IMM {
        rd: u8,
        rs1: u8,
        shamt: u8,
        is_srai: bool,
    },
    STORE {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    BRANCH {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    OP {
        rd: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    ECALL,
    EBREAK,
    MUL {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    DIV {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    DIVU {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    REM {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    REMU {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
}
