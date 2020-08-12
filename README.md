# LC-3 Machine Simulator

## Desgin

Compile the simulator
```
cargo build
```

Run the machine simulator, given a specific lc3 trace file
```
cargo run trace/<lc3_trace_file>.asm
```

Run the simulator with debugging mode, given a specific lc3 trace file
```
cargo run --debugger trace/<lc3_trace_file>.asm
```

Run the visualizer
```
npm run serve
```

TODO Functionalities <br />
* fully featured LC3 Architecture
* command-line debugger and terminal syntax highlight
* WASM-based web simulator

TODO docs 
* docs for LC3 ISA
* docs for command-line virtual debugging
* docs for playing the simulator with WASM

TODO promotion
* promote to CS/ECE 252

References <br />
* [Write Your Own Virtual Machine](https://justinmeiners.github.io/lc3-vm/#1:16)
* [KuldeepSinh first Rust implementation](https://github.com/KuldeepSinh/lc3_vm)
* for now, may be reference more in the future!

