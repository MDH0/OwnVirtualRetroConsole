use crate::mmu::MMU;

pub enum Instruction {
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
    RET,
}

pub struct CPU {
    registers: [u8; 8],
    current_instruction: Instruction,
    program_counter: usize,
    mmu: MMU,
}

impl CPU {
    pub fn init(mmu: MMU) -> Self {
        CPU {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            current_instruction: Instruction::NOP,
            program_counter: 0,
            mmu,
        }
    }
}

impl CPU {
    fn execute_instruction(&mut self) {
        match self.current_instruction {
            Instruction::ADD{register, value} => {
                if let 1..=8 = register {
                    self.registers[register] += value;
                }
                else {
                    panic!("Invalid register!")
                }
            },
            Instruction::SUB{register, value} => {
                if let 1..=8 = register {
                    self.registers[register] -= value;
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::MOV{register, target_address } => {
                if let 1..=8 = register {
                    todo!()
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::LD{address, target_register} => {
                if let 1..=8 = target_register {
                    todo!()
                }
                else {
                    panic!("Invalid register!")
                }
            }
            Instruction::NOP => return,
            Instruction::RET => {
                self.mmu.cleanup_function();
            }
        }
        self.current_instruction = self.mmu.get_next_instruction()
    }
}