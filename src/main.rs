fn main() {
}

static mut Memory: [u8; 4096] = [0; 4096];

static mut Stack: [u16, 16] = [0; 16];

static mut Display: [bool, 64][bool, 32] = [false; 64][false; 32];

struct Registers {
    i: u16,
    pc: u16,
    sp: u16,
    dt: u8,
    st: u8,
    
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8
}

enum Opcode {
    00E0, //Clear Scren (CLS)
    00EE, //Return from Subroutine (RET)
    1(u16);
} 