use crate::{
    cpu::{CPU, errors::CPUError},
    itxs::Instruction,
};

impl CPU {
    pub fn execute(&mut self, instr: Instruction) -> Result<(), CPUError> {
        use Instruction::*;
        match instr {
            LUI { rd, imm } => {
                self.write_reg(rd as usize, imm);
                Ok(())
            }

            AUIPC { rd, imm } => {
                let val = self.pc.wrapping_add(imm);
                self.write_reg(rd as usize, val);
                Ok(())
            }

            JAL { rd, imm } => {
                let ret = self.pc.wrapping_add(4);
                self.write_reg(rd as usize, ret);

                self.pc = self.pc.wrapping_add(imm as u32);
                Ok(())
            }

            JALR { rd, rs1, imm } => {
                let ret = self.pc.wrapping_add(4);
                self.write_reg(rd as usize, ret);
                let base = self.read_reg(rs1 as usize);
                let target = base.wrapping_add(imm as u32) & !1;
                self.pc = target;
                Ok(())
            }
            OP_IMM { rd, rs1, imm } => {
                let res = self.read_reg(rs1 as usize).wrapping_add(imm as u32);
                self.write_reg(rd as usize, res);
                Ok(())
            }
            SHIFT_IMM {
                rd,
                rs1,
                shamt,
                is_srai,
            } => {
                let v = self.read_reg(rs1 as usize);
                let res = if is_srai { v >> shamt } else { v << shamt };
                self.write_reg(rd as usize, res);
                Ok(())
            }
            LOAD { rd, rs1, imm } => {
                let addr = self.read_reg(rs1 as usize).wrapping_add(imm as u32);
                let word = crate::memory::Memory::load_word(&mut self.memory, addr)?;
                self.write_reg(rd as usize, word);
                Ok(())
            }
            STORE { rs1, rs2, imm } => {
                let addr = self.read_reg(rs1 as usize).wrapping_add(imm as u32);
                let val = self.read_reg(rs2 as usize);
                crate::memory::Memory::store_word(&mut self.memory, addr, val)?;
                Ok(())
            }
            BRANCH { rs1, rs2, imm } => {
                let v1 = self.read_reg(rs1 as usize);
                let v2 = self.read_reg(rs2 as usize);
                if v1 == v2 {
                    self.pc = self.pc.wrapping_add(imm as u32);
                }
                Ok(())
            }
            OP {
                rd,
                rs1,
                rs2,
                funct7,
            } => {
                let a = self.read_reg(rs1 as usize);
                let b = self.read_reg(rs2 as usize);
                let res = match funct7 {
                    0x00 => a.wrapping_add(b),
                    0x20 => a.wrapping_sub(b),
                    _ => return Err(CPUError::InvalidRegister(funct7 as usize)),
                };
                self.write_reg(rd as usize, res);
                Ok(())
            }
            ECALL | EBREAK => Ok(()),
            MUL { rd, rs1, rs2 } => {
                let res = self
                    .read_reg(rs1 as usize)
                    .wrapping_mul(self.read_reg(rs2 as usize));
                self.write_reg(rd as usize, res);
                Ok(())
            }
            DIV { rd, rs1, rs2 } => {
                let a = self.read_reg(rs1 as usize) as i32;
                let b = self.read_reg(rs2 as usize) as i32;
                let res = if b == 0 { -1 } else { a.wrapping_div(b) };
                self.write_reg(rd as usize, res as u32);
                Ok(())
            }
            DIVU { rd, rs1, rs2 } => {
                let a = self.read_reg(rs1 as usize);
                let b = self.read_reg(rs2 as usize);
                let res = if b == 0 { u32::MAX } else { a.wrapping_div(b) };
                self.write_reg(rd as usize, res);
                Ok(())
            }
            REM { rd, rs1, rs2 } => {
                let a = self.read_reg(rs1 as usize) as i32;
                let b = self.read_reg(rs2 as usize) as i32;
                let res = if b == 0 { a } else { a.wrapping_rem(b) };
                self.write_reg(rd as usize, res as u32);
                Ok(())
            }
            REMU { rd, rs1, rs2 } => {
                let a = self.read_reg(rs1 as usize);
                let b = self.read_reg(rs2 as usize);
                let res = if b == 0 { a } else { a.wrapping_rem(b) };
                self.write_reg(rd as usize, res);
                Ok(())
            }
        }
    }
}
