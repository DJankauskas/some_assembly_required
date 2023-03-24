use std::fmt::Display;

// Valid registers from 0 to 15
#[derive(Clone, Copy, Debug)]
pub struct Register(u8);

impl Register {
    pub fn new(reg: u8) -> Register {
        if reg > 15 {
            panic!("Invalid register number {reg}: must be between 0 and 15");
        }
        Register(reg)
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Nop,
    Mov(Register, u64),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Print,
    Printr,
    Copy(Register, Register),
    // If register 0 == register 1, jump to instruction # stored in register 2
    Beq(Register, Register, Register),
    Bne(Register, Register, Register),
    J(Register),
    Halt,
    Write(Register, Box<Instruction>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Mov(reg, val) => write!(f, "mov {}, {}", reg, val),
            Instruction::Add(dest, src1, src2) => {
                write!(f, "add {}, {}, {}", dest, src1, src2)
            }
            Instruction::Sub(dest, src1, src2) => {
                write!(f, "sub {}, {}, {}", dest, src1, src2)
            }
            Instruction::Mul(dest, src1, src2) => {
                write!(f, "mul {}, {}, {}", dest, src1, src2)
            }
            Instruction::Print => write!(f, "print"),
            Instruction::Printr => write!(f, "printr"),
            Instruction::Copy(dest, src) => write!(f, "copy {}, {}", dest, src),
            Instruction::Beq(reg1, reg2, reg3) => {
                write!(f, "beq {}, {}, {}", reg1, reg2, reg3)
            }
            Instruction::Bne(reg1, reg2, reg3) => {
                write!(f, "bne {}, {}, {}", reg1, reg2, reg3)
            }
            Instruction::J(reg) => write!(f, "j {}", reg),
            Instruction::Halt => write!(f, "halt"),
            Instruction::Write(dst, instr) => write!(f, "write {}, [{}]", dst, instr),
        }
    }
}