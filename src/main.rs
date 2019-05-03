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

        match (h1, l1, h2, l2) {
            
            (0x0, 0x0, 0xE, 0x0) => chip8::clear(),

            (0x0, 0x0, 0xE, 0xE) => chip8::ret(),

            (0x1,  _ ,  _ ,  _ ) => chip8::jump(l1, h2, l2),

            (0x2,  _ ,  _ ,  _ ) => chip8::call(l1, h2, l2),

            (0x3,  _ ,  _ ,  _ ) => chip8::skipequal(l1, h2, l2),

            (0x4,  _ ,  _ ,  _ ) => chip8::skipnotequal(l1, h2, l2),

            (0x5,  _ ,  _ , 0x0) => chip8::skipequal(l1, h2),

            (0x6,  _ ,  _ ,  _ ) => chip8::load(l1, h2, l2),

            (0x7,  _ ,  _ ,  _ ) => chip8::add(l1, h2, l2),

            (0x8,  _ ,  _ , 0x0) => chip8::load(l1, h2),

            (0x8,  _ ,  _ , 0x1) => chip8::or(l1, h2),

            (0x8,  _ ,  _ , 0x2) => chip8::and(l1, h2),

            (0x8,  _ ,  _ , 0x3) => chip8::xor(l1, h2),

            (0x8,  _ ,  _ , 0x4) => chip8::add(l1, h2),

            (0x8,  _ ,  _ , 0x5) => chip8::sub(l1, h2),

            (0x8,  _ ,  _ , 0x6) => chip8::shr(l1, h2),

            (0x8,  _ ,  _ , 0x7) => chip8::subn(l1, h2),

            (0x8,  _ ,  _ , 0xE) => chip8::shl(l1, h2),

            (0x9,  _ ,  _ , 0x0) => chip8::sne(l1, h2),

            (0xA,  _ ,  _ ,  _ ) => chip8::ld(l1, h2, l2),

            (0xB,  _ ,  _ ,  _ ) => chip8::jp(l1, h2, l2),

            (0xC,  _ ,  _ ,  _ ) => chip8::rnd(l1, h2, l2),

            (0xD,  _ ,  _ ,  _ ) => chip8::drw(l1, h2, l2),

            (0xE,  _ , 0x9, 0xE) => chip8::skp(l1),

            (0xE,  _ , 0xA, 0x1) => chip8::sknp(l1),

            (0xF,  _ , 0x0, 0x7) => chip8::ld(l1, dt),

            (0xF,  _ , 0x0, 0xA) => chip8::ld(l1, key_press),

            (0xF,  _ , 0x1, 0x5) => chip8::ld(dt, l1),

            (0xF,  _ , 0x1, 0x8) => chip8::ld(st, l1),

            (0xF,  _ , 0x1, 0xE) => chip8::add(dt, l1),

            (0xF,  _ , 0x2, 0x9) => chip8::load(),

            (0xF,  _ , 0x3, 0x3) => chip8::dumpbcd(),

            (0xF,  _ , 0x5, 0x5) => chip8::regdump(),

            (0xF,  _ , 0x6, 0x5) => chip8::regload(),
            
            _ => println!("{:x?} is not an instruction", [h1, l1, h2, l2])
        }
    }
}
