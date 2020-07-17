/* 
This is the LC-3 Memory Module
*/

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

    // write to the moery given an address and value
    pub fn write(&mut self, addr: u16, value: u16) {
        self.cells[addr as usize] = value;
    }
}

// implement clone trait for memory 
impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
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