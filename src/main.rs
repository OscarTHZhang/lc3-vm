pub mod memory;
fn main() {
    let mut mem = memory::Memory::new(); 
    mem.write(12 as u16, 15 as u16);
    let x = mem.read(12 as u16);
    println!("Hello, world!");
    println!("Value is :{}", x);
}
