// specific file to handle trap instructions
use crate::register::RegFile;
use crate::memory::Memory;

use std::io::Read;
use std::io::Write;


// TRAP code constants
pub const GETC: u16  = 0x20;
pub const OUT: u16   = 0x21;
pub const PUTS: u16  = 0x22;
pub const IN: u16    = 0x23;
pub const PUTSP: u16 = 0x24;
pub const HALT: u16  = 0x25;

/**
 * interacting with I/O
 */
pub fn instr_trap(instr: u16, reg_file: &mut RegFile, mem: &mut Memory) {
    let trap_vector = instr & 0xff;
    match trap_vector {
        GETC => {
            let mut buffer = [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            reg_file.update_reg(0, buffer[0] as u16);
            // no update for condition flag because this is a sys trap
        },
        OUT => {
            // output the char from register 0
            let c = reg_file.read_reg(0) as u8;
            print!("{}", c as char);
            std::io::stdout().flush().expect("Flushed.");
        },
        PUTS => {

        },
        IN => {

        },
        PUTSP => {

        },
        HALT => {

        },
        _ => {
            panic!("invalid trap vector!");
        },
    }
}