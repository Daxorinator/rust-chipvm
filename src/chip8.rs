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

fn clear() {
    //clear display
}

fn ret() {
    //Return from subroutine
}

fn call() {

}

fn skipequal() {

}

fn skipnotequal() {

}

fn load(x, y) {

}

fn add() {

}

fn or() {

}

fn and() {

}

fn xor() {

}

fn sub() {

}

fn shr() {

}

fn subn() {

}

fn shl() {

}

fn jump() {

}

fn rand() {

}

fn draw() {

}

fn push() {

}

fn npush() {

}