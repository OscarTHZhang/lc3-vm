pub const PC_START: u16 = 0x3000;
pub const PC_REG: u16 = 8;
pub const COND_REG: u16 = 9;

pub enum CondFlag {
    POS = 1 << 0, // Postive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}


#[derive(Debug)]
pub struct RegFile {
    pub r_r0: u16, // expose r0 to outside
    r_r1: u16,
    r_r2: u16,
    r_r3: u16,
    r_r4: u16,
    r_r5: u16,
    r_r6: u16,
    pub r_r7: u16, // expose R7 for subroutine purpose
    pub r_pc: u16, // expose pc for running instruction
    pub r_cond: u16, // conditional register for compare
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

    // the following functions are only for u16 arithmatics
    pub fn update_reg(&mut self, reg: u16, val: u16) {
        // map the update for the registers
        match reg {
            0 => self.r_r0 = val,
            1 => self.r_r1 = val,
            2 => self.r_r2 = val,
            3 => self.r_r3 = val,
            4 => self.r_r4 = val,
            5 => self.r_r5 = val,
            6 => self.r_r6 = val,
            7 => self.r_r7 = val,
            PC_REG => self.r_pc = val,
            COND_REG => self.r_cond = val,
            _ => panic!("Invalid Register in RegFile!"),
        }
    }

    pub fn read_reg(&mut self, reg: u16) -> u16 {
        // map registers
        match reg {
            0 => self.r_r0, 
            1 => self.r_r1,
            2 => self.r_r2,
            3 => self.r_r3,
            4 => self.r_r4,
            5 => self.r_r5,
            6 => self.r_r6,
            7 => self.r_r7,
            PC_REG => self.r_pc,
            COND_REG => self.r_cond,
            _ => panic!("Invalid Register in RegFile!"),
        }
    }

    pub fn update_cond_flag(&mut self, reg: u16) {
        let val = self.read_reg(reg);
        if val == 0 {
            self.update_reg(COND_REG, CondFlag::ZRO as u16);
        } else if (val >> 15) != 0 {
            self.update_reg(COND_REG, CondFlag::NEG as u16);
        } else {
            self.update_reg(COND_REG, CondFlag::POS as u16);
        }
    }
}

#[cfg(test)]
mod register_test {

    use super::*;

    #[test]
    fn flag_zero() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(1, 0);
        reg_file.update_cond_flag(1);
        let value = reg_file.read_reg(COND_REG);
        assert_eq!(CondFlag::ZRO as u16, value);
    }

    #[test]
    fn flag_positive() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(3, 12);
        reg_file.update_cond_flag(3);
        let value = reg_file.read_reg(COND_REG);
        assert_eq!(CondFlag::POS as u16, value);
    }

    #[test]
    fn flag_negative() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(5, 0xE00F);
        reg_file.update_cond_flag(5);
        let value = reg_file.read_reg(COND_REG);
        assert_eq!(CondFlag::NEG as u16, value);
    }
}