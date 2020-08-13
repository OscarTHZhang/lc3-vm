import * as wasm from "hello-wasm-pack";

// wasm.greet();

const select_button = document.getElementById("select-file");

select_button.addEventListener("click", event => {
    wasm.greet();
});
