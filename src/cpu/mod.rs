use crate::{
    cpu::errors::CPUError,
    decoder::decode,
    itxs::Instruction,
    memory::{Memory, errors::MemoryError},
};

mod errors;
mod exec;
mod tests;

pub type StepResult = Instruction;

impl Default for CPU {
    fn default() -> Self {
        CPU {
            regs: [0; 32],
            pc: 0,
            memory: Memory::default(),
        }
    }
}

pub struct CPU {
    regs: [u32; 32],
    pc: u32,
    pub memory: Memory,
}

impl CPU {
    pub fn new(mem_size: usize) -> Result<Self, MemoryError> {
        Ok(CPU {
            regs: [0; 32],
            pc: 0,
            memory: Memory::new(mem_size)?,
        })
    }

    pub fn read_reg(&self, idx: usize) -> u32 {
        if idx == 0 { 0 } else { self.regs[idx] }
    }

    pub fn write_reg(&mut self, idx: usize, value: u32) {
        if idx != 0 && idx < 32 {
            self.regs[idx] = value;
        }
    }

    pub fn advance_pc(&mut self, offset: u32) {
        self.pc = self.pc.wrapping_add(offset);
    }

    pub fn step(&mut self) -> Result<StepResult, CPUError> {
        let word = self.memory.fetch(self.pc)?;
        let insn = decode(word)?;
        self.advance_pc(4);
        Ok(insn)
    }
}
