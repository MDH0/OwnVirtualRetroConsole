#[derive(Default)]
pub struct RAM {
    memory: [u8; 4096], //TODO: How much memory should the console have?? Right now random value from Github Copilot
}

impl RAM {
    pub fn write(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read(&self, address: usize) -> u8 {
        self.memory[address]
    }
}