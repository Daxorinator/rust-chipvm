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

pub fn clear() {
    //clear display
}

pub fn ret() {
    //Return from subroutine
}

pub fn call() {
    //Call subroutine
}

pub fn skipequal() {
    //Skip if equal
}

pub fn skipnotequal() {
    //Skip not equal
}

pub fn load(x, y) {
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

pub fn jump() {

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