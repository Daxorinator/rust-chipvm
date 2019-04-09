mod chip8;
use std::fs;

fn main() {
    let mut registers: chip8::Registers = Default::default();
    let mut STACK: [u16; 16] = [0x000000000000000; 16];
    let mut DISPLAY: [[bool; 64]; 32] = [[false; 64]; 32];
    let mut MEMORY: Vec<u8> = fs::read("rom").unwrap();
}
