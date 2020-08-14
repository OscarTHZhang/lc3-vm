/* 
This is the LC-3 Memory Module
*/
use std::fmt;
use prettytable::*;

// define the maximum size for memeory
pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

// define memory and implement it
#[derive(Copy)]
pub struct Memory {
    pub cells: [u16; MEMORY_SIZE],
}

impl Memory {

    // initialize a memory block
    pub fn new() -> Memory {
        Memory {
            cells: [0; MEMORY_SIZE],
        }
    }

    // read from the memory given an address
    pub fn read(&self, addr: u16) -> u16 {
        self.cells[addr as usize]
    }

    // write to the memory given an address and value
    pub fn write(&mut self, addr: u16, value: u16) {
        self.cells[addr as usize] = value;
    }

    // show the memory content in the terminal using a pretty table
    pub fn show_content(&self) {
        let mut table = prettytable::Table::new();
        table.add_row(row!["address", "data"]);
        let mut addr = 0 as usize;
        while addr < MEMORY_SIZE {
            table.add_row(prettytable::Row::new(vec![
                Cell::new(format!("0x{:x}", addr).as_str()),
                Cell::new(format!("0x{:x}", self.cells[addr]).as_str()),
            ]));
            addr += 1;
        }
        table.printstd();
    }
}

// implement clone trait for memory 
impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
    }
}

// display memory content in terminal directly
impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Displaying memory content...\n=====================\n")?;
        for i in 0..self.cells.len() {
            write!(f, "address 0x{:x?} => 0x{:x?}\n", i as u16, self.cells[i] as u16)?;
        }
        write!(f, "=====================\n")?;
        Ok(())
    }
}


// test for memory
#[cfg(test)]
mod memory_test {

    use super::*;

    #[test]
    fn write_to_mem() {
        let mut mem = Memory::new();
        mem.write(0x0E61, 10);
        let value = mem.read(0x0E61);
        assert_eq!(value, 10);
    }

}