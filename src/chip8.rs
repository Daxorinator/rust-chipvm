#[derive(Default)]
pub struct Registers {
    pub i: u16,
    pub pc: u16,
    pub sp: u16,
    pub dt: u8,
    pub st: u8,

    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
}

pub fn clear(&mut display: &mut [[bool; 64]; 32]) -> () {
    let display = [[false; 64]; 32];
}

pub fn ret(&mut registers: &mut Registers, stack: [u16; 16]) {
    //Return from subroutine
    registers.pc = stack[0];
    registers.sp = registers.sp - 1;
}

pub fn call(l1: u8, h2: u8, l2: u8) {
    //Call subroutine
}

pub fn skipequal(l1: u8, h2: u8, l2: u8) {
    //Skip if equal
}

pub fn skipnotequal(l1: u8, h2: u8, l2: u8) {
    //Skip not equal
}

pub fn load(l1: u8, h2: u8, l2: u8) {
    //Load register
}

pub fn add() {
    //Add register
}

pub fn or() {
    //BITWISE OR
}

pub fn and() {
    //BITWISE AND
}

pub fn xor() {
    //BITWISE XOR
}

pub fn sub() {
    //Subtract register
}

pub fn shr() {

}

pub fn subn() {

}

pub fn shl() {

}

pub fn jump(l1, h2, l2) {

}

pub fn rand() {

}

pub fn draw() {

}

pub fn push() {

}

pub fn npush() {

}

pub fn lpush() {

}

pub fn dumpbcd() {
    
}

pub fn regdump() {

}

pub fn regload() {

}