use crate::ram::RAM;

enum Instruction {
    ADD {
        register: usize,
        value: u8,
    },
    SUB {
        register: usize,
        value: u8,
    },
    MOV {
        register: usize,
        target_address: usize,
    },
    LD {
        address: usize,
        target_register: usize,
    },
    NOP,

}

struct CPU {
    registers: [u8; 8],
    current_instruction: Instruction,
    ram: RAM,
}

impl CPU {
    fn execute_instruction(&mut self) {
        match self.current_instruction {
            Instruction::ADD(register, value) => {
                if let 1..=8 = register {
                    self.registers[register] += value;
                }
                else {
                    panic!("Invalid register!")
                }
            },
            Instruction::SUB(register, value) => {
                if let 1..=8 = register {
                    self.registers[register] -= value;
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::MOV(register, value) => {
                if let 1..=8 = register {
                    self.ram.write(value, self.registers[register]);
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::LD(address, register) => {
                if let 1..=8 = register {
                    self.registers[register] = self.ram.read(address);
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::NOP => return,
        }
    }
}