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
    let base_reg = (instr >> 6) & 0x7;
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

    // testing branch positive
    #[test]
    fn test_instr_brp() {
        let mut reg_file = RegFile::new();
        let add: u16 = 0b0101001001100001;
        let brp: u16 = 0b0000001000111000; // PC <- PC + 000111000
        instr_add(add, &mut reg_file);
        let pc_val = reg_file.read_reg(PC_REG);
        instr_brx(brp, &mut reg_file);
        let value = reg_file.read_reg(PC_REG);
        assert_eq!(value - pc_val, 0b000111000);
    }

    // test unconditional branch / jump
    #[test]
    fn test_instr_jmp() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(1, 0b0111 as u16);
        let jmp: u16 = 0b1100000001000000;
        instr_jmp(jmp, &mut reg_file);
        let value = reg_file.read_reg(PC_REG);
        assert_eq!(value, 0b0111 as u16);
    }

    // test jsr label (jump to label)
    #[test]
    fn test_instr_jsr() {
        let mut reg_file = RegFile::new();
        let jsr: u16 = 0b0100100011011010;
        let pc_val = reg_file.read_reg(PC_REG);
        instr_jsr(jsr, &mut reg_file);
        let r7 = reg_file.read_reg(7);
        let value = reg_file.read_reg(PC_REG);
        assert!((r7 == pc_val) && (value - pc_val == 0b00011011010 as u16));
    }

    #[test]
    fn test_instr_jsrr() {
        let mut reg_file = RegFile::new();
        reg_file.update_reg(2, 0b011101 as u16);
        let jsrr: u16 = 0b0100000010000000;
        let pc_val = reg_file.read_reg(PC_REG);
        instr_jsr(jsrr, &mut reg_file);
        let r7 = reg_file.read_reg(7);
        let value = reg_file.read_reg(PC_REG);
        assert!((r7 == pc_val) && (value == 0b011101 as u16));
    }

    #[test]
    fn test_instr_ld() {
        let mut reg_file = RegFile::new();
        let mut mem = Memory::new();
        let location = (0x3000 + 0x25) as u16;
        mem.write(location, 0x333 as u16);
        let ld = (0b0010001000000000 | 0x25) as u16;
        instr_ld(ld, &mut reg_file, &mut mem);
        let value = reg_file.read_reg(1);
        assert_eq!(value, 0x333 as u16);
    }

    // test ldi instr
    #[test]
    fn test_instr_ldi() {
        let mut reg_file = RegFile::new();
        let mut mem = Memory::new();
        let addr = (0x3000 + 0x35) as u16;
        mem.write(addr, 0x3333 as u16);
        mem.write(0x3333, 0x1528);
        let ldi = (0b1010001000000000 | 0x35) as u16;
        instr_ldi(ldi, &mut reg_file, &mut mem);
        let value = reg_file.read_reg(1);
        assert_eq!(value, 0x1528 as u16);
    }

    // test ldr instr
    #[test]
    fn test_instr_ldr() {
        let mut reg_file = RegFile::new();
        let mut mem = Memory::new();
        let addr = (0x2099 + 0x00F) as u16;
        mem.write(addr, 0x5555);
        reg_file.update_reg(5, 0x2099);
        let ldr = (0b0110010101000000 | 0x00F) as u16;
        instr_ldr(ldr, &mut reg_file, &mut mem);
        let value = reg_file.read_reg(2);
        println!("{}", value);
        assert_eq!(value, 0x5555);
    }

    // test for lea
    #[test]
    fn test_instr_lea() {
        let mut reg_file = RegFile::new();
        // let mut mem = Memory::new();
        let addr = (0x3000 + 0x003B) as u16;
        let lea = (0b1110110000000000 | 0x003B) as u16;
        instr_lea(lea, &mut reg_file);
        let value = reg_file.read_reg(6);
        assert_eq!(value, addr);
    }

    #[test]
    fn test_instr_not() {
        let mut reg_file = RegFile::new();
        let not = 0b1001001001111111 as u16;
        reg_file.update_reg(1, 0xFFFF);
        instr_not(not, &mut reg_file);
        let value = reg_file.read_reg(1);
        assert_eq!(value, 0x0000 as u16);
    }

}