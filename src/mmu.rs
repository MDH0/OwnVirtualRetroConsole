use crate::ram::RAM;
use crate::file_data::FileData;
use crate::cpu::Instruction;

pub struct MMU {
    ram: RAM,
    vram_size: usize,
    static_data_size: usize,
}

impl MMU {
    pub fn init(ram: RAM, game_path: String) -> Self {
        let game_data = FileData::open(game_path);
        MMU {
            ram,
            vram_size: 0,
            static_data_size: game_data.static_data_length as usize,
        }
    }

    pub fn cleanup_function(&self) {
        todo!()
    }

    pub fn get_next_instruction(&self) -> Instruction {
        todo!()
    }

    pub fn get_static_data(&self, _address: usize) -> u8 {
        todo!()
    }
}