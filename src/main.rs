// //extern crate wabt;
// use std::fs::File;
// use std::io::Read;
// use wasmparser::{Parser, Chunk, Payload::*};


// fn read_wasm_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
//     let mut file = File::open(file_path)?;
//     let mut wasm_data = Vec::new();
//     file.read_to_end(&mut wasm_data)?;
//     Ok(wasm_data)
// }

// fn main() -> Result<(), Box<dyn std::error::Error>>{
//     //let binary_bytes = include_bytes!("hello_cargo.wasm");
    
//     //let module = wabt::Module::read_binary(binary_bytes).expect("Failed to parse Wasm module");
    
//     // Access and work with the parsed Wasm module data here
//     let wasm_file_path = "hello_cargo.wasm";
//     let wasm_binary = read_wasm_file(wasm_file_path)?;
//     //println!("File contents: {}", wasm_binary);

//     let mut parser = Parser::new(wasm_binary);

//     for payload in parser.parse_all()

//     Ok(())
// }

use std::io::Read;
use std::fs::File;
use wasmparser::{Parser, Chunk, Payload::*, types};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file_path = "src/hello_cargo.wasm";
    let file = File::open(wasm_file_path)?;
    parse(file)?;

    Ok(())
}


fn parse(mut reader: impl Read) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let parser = Parser::new(0);

    for payload in parser.parse_all(&buf) {
        match payload? {
            // Sections for WebAssembly modules
            Version { .. } => { /* ... */ }
            TypeSection(types) => { }
            ImportSection(imports) => { /* ... */ }
            FunctionSection(types) => { /* ... */ }
            TableSection(tables) => { /* ... */ }
            MemorySection(memories) => { /* ... */ }
            TagSection(tags) => { /* ... */ }
            GlobalSection(globals) => { /* ... */ }
            ExportSection(exports) => { /* ... */ }
            StartSection { .. } => { /* ... */ }
            ElementSection(elements) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(data) => { /* ... */ }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart { .. } => { /* ... */ }
            CodeSectionEntry(body) => {
                // here we can iterate over `body` to parse the function
                // and its locals
            }

            // Sections for WebAssembly components
            ModuleSection { .. } => { /* ... */ }
            InstanceSection(_) => { /* ... */ }
            CoreTypeSection(_) => { /* ... */ }
            ComponentSection { .. } => { /* ... */ }
            ComponentInstanceSection(_) => { /* ... */ }
            ComponentAliasSection(_) => { /* ... */ }
            ComponentTypeSection(_) => { /* ... */ }
            ComponentCanonicalSection(_) => { /* ... */ }
            ComponentStartSection { .. } => { /* ... */ }
            ComponentImportSection(_) => { /* ... */ }
            ComponentExportSection(_) => { /* ... */ }

            CustomSection(_) => { /* ... */ }

            // most likely you'd return an error here
            UnknownSection { id, .. } => { /* ... */ }

            // Once we've reached the end of a parser we either resume
            // at the parent parser or the payload iterator is at its
            // end and we're done.
            End(_) => {}
        }
    }

    Ok(())
}