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

fn cls() {
    //clear display
}

fn ret() {
    //Return from subroutine
}

fn call() {
    //Call a subroutine
}

fn se() {
    //Skip next instruction if Vx = kk
}

