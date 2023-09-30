// PARSED CODE
// fn main() {
//     println!("Hello, world!");
// }

use std::fs::File;
use std::io::Read;
use wasmparser::{types, Chunk, DataKind, Parser, Payload::*, VisitOperator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file_path = "src/lib/heapsort.wasm";
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
            Version { .. } => {
                println!("====== Module");
            }
            TypeSection(types) => {}
            ImportSection(imports) => {
                for import in imports {
                    let import = import?;
                    println!("  Import {}::{}", import.module, import.name);
                }
            }
            FunctionSection(types) => { /* ... */ }
            TableSection(tables) => { /* ... */ }
            MemorySection(memories) => { /* ... */ }
            TagSection(tags) => { /* ... */ }
            GlobalSection(globals) => { /* ... */ }
            ExportSection(exports) => {
                for export in exports {
                    let export = export?;
                    println!("  Export {} {:?}", export.name, export.kind);
                }
            }
            StartSection { .. } => { /* ... */ }
            ElementSection(elements) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(data) => {
                for item in data {
                    let item = item?;
                    if let DataKind::Active { offset_expr, .. } = item.kind {
                        for op in offset_expr.get_operators_reader() {
                            op?;
                        }
                    }
                }
            }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart { count, range, size } => {
                println!("{}", count);
                println!("{}", size);
            }

            CodeSectionEntry(body) => {
                let mut reader = body.get_binary_reader();
                // for val in 0..reader.read_var_u32()? {
                //     reader.read_var_u32()?;
                //     println!("val {}", val);
                //     //reader.read::<wasmparser::ValType>()?;
                //     println!("Data Val Type {}", reader.read::<wasmparser::ValType>()?);
                // }
                while !reader.eof() {
                    let op = reader.read_operator();
                    println!("Operator: {:?}", op)
                }
                // while !reader.eof() {
                //     reader.visit_operator(&mut NopVisit)?;
                // }
            }

            // Sections for WebAssembly components
            ModuleSection { .. } => { /* ... */ }
            InstanceSection(s) => {
                for item in s {
                    item?;
                }
            }
            CoreTypeSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentSection { .. } => { /* ... */ }
            ComponentInstanceSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentAliasSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentTypeSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentCanonicalSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentStartSection { .. } => { /* ... */ }
            ComponentImportSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentExportSection(s) => {
                for item in s {
                    item?;
                }
            }

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
