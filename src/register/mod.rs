
pub const PC_START: u16 = 0x3000;
pub const PC_NUM: u16 = 8;
pub const COND_NUM: u16 = 9;

#[derive(Debug)]
pub struct RegFile {
    pub r_r0: u16,
    pub r_r1: u16,
    pub r_r2: u16,
    pub r_r3: u16,
    pub r_r4: u16,
    pub r_r5: u16,
    pub r_r6: u16,
    pub r_r7: u16,
    pub r_pc: u16,
    pub r_cond: u16, // conditional register
}


impl RegFile {
    // initialize the register file
    pub fn new() -> RegFile {
        RegFile {
            r_r0: 0,
            r_r1: 0,
            r_r2: 0,
            r_r3: 0,
            r_r4: 0,
            r_r5: 0,
            r_r6: 0,
            r_r7: 0,
            r_pc: PC_START,
            r_cond: 0, // conditional register 
        }
    }

    pub fn update_reg(&mut self, reg: u16, val: u16) {
        // map the update for the registers
        match reg {
            0 => panic!("R0 should never be changed!"), // R0 should never be changed!
            1 => self.r_r1 = val,
            2 => self.r_r2 = val,
            3 => self.r_r3 = val,
            4 => self.r_r4 = val,
            5 => self.r_r5 = val,
            6 => self.r_r6 = val,
            7 => self.r_r7 = val,
            PC_NUM => self.r_pc = val,
            COND_NUM => self.r_cond = val,
            _ => panic!("Invalid Register in RegFile!"),
        }
    }

    pub fn read_reg(&mut self, reg: u16) -> u16 {
        // map registers
        match reg {
            0 => panic!("R0 should never be changed!"), // R0 should never be changed!
            1 => self.r_r1,
            2 => self.r_r2,
            3 => self.r_r3,
            4 => self.r_r4,
            5 => self.r_r5,
            6 => self.r_r6,
            7 => self.r_r7,
            PC_NUM => self.r_pc,
            COND_NUM => self.r_cond,
            _ => panic!("Invalid Register in RegFile!"),
        }
    }
}
