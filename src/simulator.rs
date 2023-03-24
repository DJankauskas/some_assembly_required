use crate::ast::Instruction;

pub struct Simulator {
    pub registers: [u64; 16],
    pub program: [Instruction; 1024],
    pub pc: usize,
    pub output: Vec<String>,
}

impl Simulator {
    pub fn new(input: &[Instruction]) -> Simulator {
        let mut program = [Instruction::Nop; 1024];
        for (i, instr) in input.iter().enumerate() {
           program[i] = *instr;
        }
        Simulator {
            registers: [0; 16],
            program,
            pc: 0,
            output: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        web_sys::console::log_1(&format!("{:?}", self.program).into());

        loop {
            let instr = self.program[self.pc];
            match instr {
                Instruction::Nop => {
                    self.pc += 1;
                }
                Instruction::Mov(reg, val) => {
                    self.registers[reg.get() as usize] = val;
                    self.pc += 1;
                }
                Instruction::Add(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize] + self.registers[src2.get() as usize];
                    self.pc += 1;
                }
                Instruction::Sub(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize] - self.registers[src2.get() as usize];
                    self.pc += 1;
                }
                Instruction::Mul(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize] * self.registers[src2.get() as usize];
                    self.pc += 1;
                }
                Instruction::Div(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize] / self.registers[src2.get() as usize];
                    self.pc += 1;
                }
                Instruction::Print => {
                    let pc = if self.pc == 0 { 1023 } else { self.pc - 1 };
                    self.output.push(self.program[pc].to_string());
                    self.pc += 1;
                }
                Instruction::Copy(dest, src) => {
                    self.program[self.registers[dest.get() as usize] as usize] = self.program[self.registers[src.get() as usize] as usize];
                    self.pc += 1;
                }
                Instruction::Beq(reg1, reg2, reg3) => {
                    if self.registers[reg1.get() as usize] == self.registers[reg2.get() as usize] {
                        self.pc = self.registers[reg3.get() as usize] as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::Bne(reg1, reg2, reg3) => {
                    if self.registers[reg1.get() as usize] != self.registers[reg2.get() as usize] {
                        self.pc = self.registers[reg3.get() as usize] as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::J(constant) => {
                    self.pc = constant as usize;
                }
                Instruction::Jr(reg) => {
                    self.pc = self.registers[reg.get() as usize] as usize;
                }
                Instruction::Halt => {
                    break;
                }
            }
        }
    }
}