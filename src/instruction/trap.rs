// specific file to handle trap instructions
use crate::register::RegFile;
use crate::memory::Memory;


// TRAP code
pub enum TrapCode {
    Getc = 0x20, // get character from keyboard input
    Out = 0x21, // output a character
    Puts = 0x22, // ouptut a word string
    In = 0x23, // input a string
    Putsp = 0x24, // output a byte string
    Halt = 0x25, // halt the program
}

/**
 * interacting with I/O
 */
pub fn instr_trap(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let trap_vector = instr & 0xff;
    
}