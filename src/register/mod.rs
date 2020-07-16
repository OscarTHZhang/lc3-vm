
pub const PC_START: u16 = 0x3000;

#[derive(Debug)]
pub struct RegFile {
    pub r_R0: u16,
    pub r_R1: u16,
    pub r_R2: u16,
    pub r_R3: u16,
    pub r_R4: u16,
    pub r_R5: u16,
    pub r_R6: u16,
    pub r_R7: u16,
    pub r_PC: u16,
    pub r_COND: u16, // conditional register
}

impl RegFile {
    // initialize the register file
    pub fn new() -> RegFile {
        RegFile {
            r_R0: 0,
            r_R1: 0,
            r_R2: 0,
            r_R3: 0,
            r_R4: 0,
            r_R5: 0,
            r_R6: 0,
            r_R7: 0,
            r_PC: PC_START,
            r_COND: 0, // conditional register 
        }
    }
}
