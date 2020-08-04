use crate::register::RegFile;
use crate::memory::Memory;
use super::*;

/**
 * parsing the instruction for "ADD" both R1 + R2 and R1 + IMMEDIATE
 */
pub fn instr_add(instr: u16, reg_file: &mut RegFile) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let sr1 = (instr >> SR1_SHIFT) & REG_MASK;
    let imm_flag = (instr >> IMM_BIT) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, IMM_BIT as u8);
        let val = imm5 as u32 + reg_file.read_reg(sr1) as u32;
        reg_file.update_reg(dr, val as u16);
    } else {
        let sr2 = instr & REG_MASK;
        let val = reg_file.read_reg(sr1) as u32 + reg_file.read_reg(sr2) as u32;
        reg_file.update_reg(dr, val as u16);
    }
    // update the conditional flag for destination register
    reg_file.update_cond_flag(dr);
}


/**
 * for AND both sr1 and sr2 and sr1 and immediate
 */
pub fn instr_and(instr: u16, reg_file: &mut RegFile) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let sr1 = (instr >> SR1_SHIFT) & REG_MASK;
    let imm_flag = (instr >> IMM_BIT) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, IMM_BIT as u8);
        let val = reg_file.read_reg(sr1);
        reg_file.update_reg(dr, val & imm5);
    } else {
        let sr2 = instr & REG_MASK;
        let val = reg_file.read_reg(sr1) & reg_file.read_reg(sr2);
        reg_file.update_reg(dr, val); 
    }
    reg_file.update_cond_flag(dr);
}


/**
 * for Branch instructions
 */
pub fn instr_brx(instr: u16, reg_file: &mut RegFile) {
    let pc_offset = sign_extend((instr) & 0x1ff, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & reg_file.r_cond != 0 {
        let next_pc = reg_file.r_pc as u32 + pc_offset as u32; // PC = 
        reg_file.r_pc = next_pc as u16;
    }
}


/**
 * Unconditional jump to base register address
 */
pub fn instr_jmp(instr: u16, reg_file: &mut RegFile) {
    let base_reg = (instr >> 6) & 0x7;
    reg_file.r_pc = reg_file.read_reg(base_reg);
}


/**
 * Unconditional jump to label or subroutine
 */
pub fn instr_jsr(instr: u16, reg_file: &mut RegFile) {
    let base_reg = (instr >> 7) & 0x7;
    let pc_offset = sign_extend(instr & 0x7ff, 11);
    let flag = (instr >> 11) & 1;
    reg_file.r_r7 = reg_file.r_pc; // update R7 to PC + 1;
    if flag != 0 {
        let val = reg_file.r_pc as u32 + pc_offset as u32;
        reg_file.r_pc = val as u16;
    } else {
        // JSRR case
        reg_file.r_pc = reg_file.read_reg(base_reg);
    }
}


pub fn instr_ld(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let address = pc_offset as u32 + reg_file.r_pc as u32;
    reg_file.update_reg(dr, mem.read(address as u16));
    reg_file.update_cond_flag(dr);
}


// indirect load (load twice)
pub fn instr_ldi(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let address = mem.read(reg_file.r_pc + pc_offset);
    reg_file.update_reg(dr, mem.read(address));
    reg_file.update_cond_flag(dr);
}


pub fn instr_ldr(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let sr1 = (instr >> SR1_SHIFT) & REG_MASK;
    let offset = sign_extend(instr & 0x3f, 6);
    let address = reg_file.read_reg(sr1) as u32 + offset as u32;
    reg_file.update_reg(dr, mem.read(address as u16));
    reg_file.update_cond_flag(dr);
}


pub fn instr_lea(instr: u16, reg_file: &mut RegFile) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val: u32 = reg_file.r_pc as u32 + pc_offset as u32;
    reg_file.update_reg(dr, val as u16);
    reg_file.update_cond_flag(dr);
}


pub fn instr_not(instr: u16, reg_file: &mut RegFile) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let sr1 = (instr >> SR1_SHIFT) & REG_MASK;
    let val = reg_file.read_reg(sr1);
    reg_file.update_reg(dr, !val);
    reg_file.update_cond_flag(dr);
}


pub fn instr_st(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let address = reg_file.r_pc as u32 + pc_offset as u32;
    mem .write(address as u16, reg_file.read_reg(dr));
}


pub fn instr_sti(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let addr1 = reg_file.r_pc as u32 + pc_offset as u32;
    let addr2 = mem.read(addr1 as u16);
    mem.write(addr2, reg_file.read_reg(dr));
}


pub fn instr_str(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let dr = (instr >> DR_SHIFT) & REG_MASK;
    let sr1 = (instr >> SR1_SHIFT) & REG_MASK;
    let offset = sign_extend(instr & 0x3f, 6);
    let address = reg_file.read_reg(sr1) as u32 + offset as u32;
    mem.write(address as u16, reg_file.read_reg(dr));
}

#[cfg(test)]
mod general_instruction_test {
    use super::*;
    use crate::memory::*;
    use crate::register::*;

    // add two registers
    #[test]
    fn test_instr_add() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(1, 3);
        reg_file.update_reg(2, 4);
        let binary_instr: u16 = 0b0001011001000010; // R3 <- R1 + R2
        instr_add(binary_instr, &mut reg_file);
        let value = reg_file.read_reg(3);
        assert_eq!(value, 7 as u16);
    }

    // add one register and an immediate value
    #[test]
    fn test_instr_add2() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(4, 13);
        let binary_instr: u16 = 0b0001010100100011; // R2 <- R4 + 3
        instr_add(binary_instr, &mut reg_file);
        let value = reg_file.read_reg(2);
        assert_eq!(value, 16 as u16);
    }

    // and two register values
    #[test]
    fn test_instr_and() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(1, 0b0011 as u16);
        reg_file.update_reg(2, 0b1110 as u16);
        let binary_instr: u16 = 0b0101011001000010;
        instr_and(binary_instr, &mut reg_file); // R3 <- R1 & R2
        let value = reg_file.read_reg(3);
        assert_eq!(value, 0b0010 as u16);
    }

    // and one register with immediate
    #[test]
    fn test_instr_and2() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(1, 0b1110 as u16);
        let binary_instr: u16 = 0b0101010001100110; // R2 <- R1 & 00110
        instr_and(binary_instr, &mut reg_file);
        let value = reg_file.read_reg(2);
        assert_eq!(value, 0b0110 as u16);
    }

    // branch zero test
    #[test]
    fn test_instr_brz() {
        let mut reg_file = RegFile::new();
        let and: u16 = 0b0101001001100000;
        let brz: u16 = 0b0000010010101011;
        instr_and(and, &mut reg_file);
        let pc_val = reg_file.read_reg(PC_REG);
        instr_brx(brz, &mut reg_file);
        let value = reg_file.read_reg(PC_REG);
        assert_eq!(value - pc_val, 0b010101011 as u16);
    }

    // branch negative test
    #[test]
    fn test_instr_brn() {
        let mut reg_file = RegFile::new();
        let add: u16 = 0b0101001001111100;
        let brn: u16 = 0b0000100001110110;
        instr_add(add, &mut reg_file);
        let pc_val = reg_file.read_reg(PC_REG);
        instr_brx(brn , &mut reg_file);
        let value = reg_file.read_reg(PC_REG);
        assert_eq!(value - pc_val, 0b001110110 as u16);
    }

}