use std::fs::File;
use std::io::Read;
use wasmparser::{types, Chunk, DataKind, Parser, Payload::*, VisitOperator};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
};
use wasmparser::OperatorsReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file_path = "src/lib/gcd.wasm";
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
            TypeSection(types) => {
                for ty in types {
                    let ty = ty?;
                    println!("  Type {:?}", ty);
                }
            }
            ImportSection(imports) => {
                for import in imports {
                    let import = import?;
                    println!("  Import {}::{} -- Import type{:?}", import.module, import.name, import.ty);
                }
            }
            FunctionSection(types) => { 
                for type_fun in types{
                    println!("Type fun {:?}", type_fun?)
                }
        /* ... */ }
            TableSection(tables) => { /* ... */ }
            MemorySection(memories) => { /* ... */ }
            TagSection(tags) => { /* ... */ }
            GlobalSection(globals) => { 
                for global in globals {
                    let global = global?;
                    println!("  Global {:?}", global);
                    println!("  Global type{:?}", global.ty);
                }
                /* ... */ }
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
                println!("====== Data");
                for item in data {
                    let item = item?;
                    println!("  Data {:?}", item.data);
                    if let DataKind::Active { offset_expr, .. } = item.kind {
                        for op in offset_expr.get_operators_reader() {
                            let op = op?;
                            println!("  Data {:?}", op);
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
                    println!("Operator: {:?}", op.unwrap())

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

// fn translate_func(reader: OperatorsReader, &mut context: Context,&mut module: Module,) -> Result<(), Box<dyn std::error::Error>> {


//     Ok(())
// }
