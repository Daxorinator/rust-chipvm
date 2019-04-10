mod chip8;
use std::fs;

fn main() {
    let mut registers: chip8::Registers = Default::default();
    let mut stack: [u16; 16] = [0x000000000000000; 16];
    let mut display: [[bool; 64]; 32] = [[false; 64]; 32];
    let memory = fs::read("rom").unwrap();

    for bytes in memory.chunks(2) {
        let h1 = bytes[0] >> 4;
        let l1 = bytes[0] & 15;

        let h2 = bytes[1] >> 4;
        let l2 = bytes[1] & 15;

        match h1 {
            0x0 => match l2 {
                0x0 => cls(),
                0xE => ret(),
                _ => println!("Not an instruction")
            }
            _ => println!("Go back and implement {:x?}", h1)
        }
    }
}
