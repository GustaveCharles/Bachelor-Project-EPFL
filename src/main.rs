//extern crate wabt;
use std::fs::File;
use std::io::Read;
use wasmparser::{Parser, Chunk, Payload::*};


fn read_wasm_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut wasm_data = Vec::new();
    file.read_to_end(&mut wasm_data)?;
    Ok(wasm_data)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //let binary_bytes = include_bytes!("hello_cargo.wasm");
    
    //let module = wabt::Module::read_binary(binary_bytes).expect("Failed to parse Wasm module");
    
    // Access and work with the parsed Wasm module data here
    let wasm_file_path = "hello_cargo.wasm";
    let wasm_binary = read_wasm_file(wasm_file_path)?;
    //println!("File contents: {}", wasm_binary);

    let mut parser = Parser::new(wasm_binary);

    for payload in parser.parse_all()

    Ok(())
}
