use crate::ast::Instruction;

pub struct Simulator {
    pub registers: [i64; 16],
    pub program: [Instruction; 1024],
    pub pc: usize,
    pub output: Vec<String>,
}

impl Simulator {
    pub fn new(input: &[Instruction]) -> Simulator {
        let mut program: Vec<Instruction> = (0..1024).map(|_| Instruction::Nop).collect();
        for (i, instr) in input.iter().enumerate() {
            program[i] = instr.clone();
        }
        Simulator {
            registers: [0; 16],
            program: program.try_into().unwrap(),
            pc: 0,
            output: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                self.output.push(format!("Error: program counter {} out of bounds. To stop execution, use a `halt` instruction.", self.pc));
                break;
            }
            let instr = self.program[self.pc].clone();
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
                        self.registers[src1.get() as usize].wrapping_add(self.registers[src2.get() as usize]);
                    self.pc += 1;
                }
                Instruction::Sub(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize].wrapping_sub(self.registers[src2.get() as usize]);
                    self.pc += 1;
                }
                Instruction::Mul(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize].wrapping_mul(self.registers[src2.get() as usize]);
                    self.pc += 1;
                }
                Instruction::Div(dest, src1, src2) => {
                    self.registers[dest.get() as usize] =
                        self.registers[src1.get() as usize].wrapping_div(self.registers[src2.get() as usize]);
                    self.pc += 1;
                }
                Instruction::Print => {
                    let pc = if self.pc == 0 { 1023 } else { self.pc - 1 };
                    self.output.push(self.program[pc].to_string());
                    self.pc += 1;
                }
                Instruction::Printr => {
                    let pc = if self.pc == 0 { 1023 } else { self.pc - 1 };
                    self.output.push(self.program[pc].to_string());
                    let mut register_output = String::new();
                    for i in 0..15 {
                        register_output += &format!("r{}: {}, ", i, self.registers[i]);
                    }
                    register_output += &format!("r15: {}", self.registers[15]);
                    self.output.push(register_output);
                    self.pc += 1;
                }
                Instruction::Copy(dest, src) => {
                    let src_addr = self.registers[src.get() as usize] as usize;
                    if src_addr >= self.program.len() {
                        self.output.push(format!("Error: copy from address {} out of bounds.", src_addr));
                        break;
                    }

                    let dest_addr = self.registers[dest.get() as usize] as usize;
                    if dest_addr >= self.program.len() {
                        self.output.push(format!("Error: copy to address {} out of bounds.", dest_addr));
                        break;
                    }

                    self.program[dest_addr] =
                        self.program[src_addr].clone();
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
                Instruction::J(reg) => {
                    let new_r15 = self.pc as i64 + 1;
                    self.pc = self.registers[reg.get() as usize] as usize;
                    self.registers[15] = new_r15;
                }
                Instruction::Rj(reg) => {
                    self.pc = self.pc + self.registers[reg.get() as usize] as usize;
                }
                Instruction::Write(dst, instr) => {
                    let dest_addr = self.registers[dst.get() as usize] as usize;
                    if dest_addr >= self.program.len() {
                        self.output.push(format!("Error: write to address {} out of bounds.", dest_addr));
                        break;
                    }
                    self.program[dest_addr] = *instr;
                    self.pc += 1;
                }
                Instruction::Halt => {
                    break;
                }
            }
        }
        if self.output.is_empty() {
            // Ensure output always shows up if a program ran
            self.output.push("<no output>".to_string());
        }
    }
}
