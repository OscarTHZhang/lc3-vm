use crate::register::RegFile;
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
    let pc_offset = sign_extend(instr >> 0x7ff, 11);
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


pub fn instr_ld(instr: u16, reg_file: &mut RegFile) {

}


pub fn instr_ldi(instr: u16, reg_file: &mut RegFile) {

}


pub fn instr_ldr(instr: u16, reg_file: &mut RegFile) {

}

