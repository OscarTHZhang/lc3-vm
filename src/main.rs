pub mod memory;
pub mod register;
pub mod instruction;

use std::env;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // println!("{:?}", args);
    // if args.len() <= 1 {
    //     println!("[ERROR] Invalid number of arguments!");
    // } else if  args.len() == 2 {
    //     println!("Run the simulator with a file provided...");
    //     normal_simulator(&args[1]);
    //     return;
    // } else if args.len() == 3 && args[1] == "debugger" {
    //     println!("Run the simulator with debugging mode...");
    //     return;
    // } else if args.len() == 3 && args[1] == "visualizer" {
    //     println!("Run the simulaotr with WASM bindings for visualizer...");
    //     return;
    // } else {
    //     println!("[ERROR] Invalid number of arguments!");
    // }
    // let mem = memory::Memory::new();
    // mem.show_content();
    let mut reg_file = register::RegFile::new();
    reg_file.update_reg(0, 12);
    print!("{}", reg_file);
}

fn normal_simulator(trace_path: &String) {
    let contents = fs::read_to_string(trace_path).expect("[ERROR] Unable to open the file!");
    println!("Code:\n{}", contents);

}

// let file = &args[1];
// let contents = fs::read_to_string(file)
//     .expect("unable to open the file!");

// println!("code:\n{} ", contents);