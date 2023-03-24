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

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Nop,
    Mov(Register, u64),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    Print,
    Copy(Register, Register),
    // If register 0 == register 1, jump to instruction # stored in register 2
    Beq(Register, Register, Register),
    Bne(Register, Register, Register),
    J(u64),
    Jr(Register),
    Halt,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Mov(reg, val) => write!(f, "mov {}, {}", reg.get(), val),
            Instruction::Add(dest, src1, src2) => {
                write!(f, "add {}, {}, {}", dest.get(), src1.get(), src2.get())
            }
            Instruction::Sub(dest, src1, src2) => {
                write!(f, "sub {}, {}, {}", dest.get(), src1.get(), src2.get())
            }
            Instruction::Mul(dest, src1, src2) => {
                write!(f, "mul {}, {}, {}", dest.get(), src1.get(), src2.get())
            }
            Instruction::Div(dest, src1, src2) => {
                write!(f, "div {}, {}, {}", dest.get(), src1.get(), src2.get())
            }
            Instruction::Print => write!(f, "print"),
            Instruction::Copy(dest, src) => write!(f, "copy {}, {}", dest.get(), src.get()),
            Instruction::Beq(reg1, reg2, reg3) => {
                write!(f, "beq {}, {}, {}", reg1.get(), reg2.get(), reg3.get())
            }
            Instruction::Bne(reg1, reg2, reg3) => {
                write!(f, "bne {}, {}, {}", reg1.get(), reg2.get(), reg3.get())
            }
            Instruction::J(constant) => write!(f, "j {}", constant), 
            Instruction::Jr(reg) => write!(f, "jr {}", reg.get()),
            Instruction::Halt => write!(f, "halt"),
        }
    }
}