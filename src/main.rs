use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::ram::RAM;

mod cpu;
mod ram;
mod mmu;
mod file_data;

fn main() {
    let ram = RAM::init();
    let mmu = MMU::init(ram, String::from(""));
    let mut cpu = CPU::init(mmu);
}