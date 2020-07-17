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
pub fn instr_brx() {
    
}
