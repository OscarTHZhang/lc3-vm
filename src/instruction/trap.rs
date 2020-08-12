// specific file to handle trap instructions
use crate::register::RegFile;
use crate::memory::Memory;

use std::io::Read;
use std::io::Write;
use std::process;


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
            let mut index = reg_file.read_reg(0) as usize;
            // error check: index out of range?
            let mut cc = mem.cells[index];
            while cc != 0x0000 {
                // one word stores two character, either of which could be null char
                let c1 = ((cc & 0xff) as u8) as char;
                if c1 == '\0' {
                    break;
                }
                print!("{}", c1);
                let c2 = ((cc >> 8) as u8) as char;
                if c2 == '\0' {
                    break;
                }
                print!("{}", c2);
                index += 1;
                cc = mem.cells[index];
            }
            std::io::stdout().flush().expect("Flushed.");
        },
        IN => {
            // input a character
            print!("Enter a character: ");
            let input = std::io::stdin()
                .bytes()
                .next()
                .and_then(|res| res.ok())
                .map(|byte| byte as u16)
                .unwrap();
            reg_file.update_reg(0, input);
        },
        PUTSP => {
            // output a byte string
            let mut index = reg_file.read_reg(0) as usize;
            let mut cc = mem.cells[index];
            while cc != 0x0000 {
                let c1 = ((cc & 0xff) as u8) as char;
                print!("{}", c1);
                let c2 = ((cc >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                cc = mem.cells[index];
            }
            std::io::stdout().flush().expect("Flushed.");
        },
        HALT => {
            println!("Halt the program.");
            std::io::stdout().flush().expect("Flushed.");
            process::exit(1);
        },
        _ => {
            panic!("invalid trap vector!");
        },
    }
}