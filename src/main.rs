// PARSED CODE
// fn main() {
//     println!("Hello, world!");
// }

use std::io::Read;
use std::fs::File;
use wasmparser::{Parser,DataKind,Chunk, Payload::*, types};

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
            Version { .. } => { println!("====== Module");}
            TypeSection(types) => { }
            ImportSection(imports) => { for import in imports {
                let import = import?;
                println!("  Import {}::{}", import.module, import.name);
            } }
            FunctionSection(types) => { /* ... */ }
            TableSection(tables) => { /* ... */ }
            MemorySection(memories) => { /* ... */ }
            TagSection(tags) => { /* ... */ }
            GlobalSection(globals) => { /* ... */ }
            ExportSection(exports) => { 
                for export in exports {
                let export = export?;
                println!("  Export {} {:?}", export.name, export.kind);
            } }
            StartSection { .. } => { /* ... */ }
            ElementSection(elements) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(data) => { for item in data {
                let item = item?;
                if let DataKind::Active { offset_expr, .. } = item.kind {
                    for op in offset_expr.get_operators_reader() {
                        op?;
                    }
                }
            } }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart { count,range,size } => {
                println!("{}", count);
                println!("{}", size);

                 }
            CodeSectionEntry(body) => {
                //println!("Code Section Entry:");
                // You can access and print the function bodies here
                //println!("  Operator: {:?}", body);
            
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