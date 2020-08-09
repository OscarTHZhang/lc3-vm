pub mod memory;
pub mod register;
pub mod instruction;

use std::env;
// use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() <= 1 {
        println!("[ERROR] Invalid number of arguments!");
    } else if  args.len() == 2 {
        println!("Run the simulator with a file provided...");
        return;
    } else if args.len() == 3 && args[1] == "debugger" {
        println!("Run the simulator with debugging mode...");
        return;
    } else if args.len() == 3 && args[1] == "visualizer" {
        println!("Run the simulaotr with WASM bindings for visualizer...");
        return;
    } else {
        println!("[ERROR] Invalid number of arguments!");
    }
}

// let file = &args[1];
// let contents = fs::read_to_string(file)
//     .expect("unable to open the file!");

// println!("code:\n{} ", contents);