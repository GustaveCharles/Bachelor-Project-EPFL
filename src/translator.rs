extern crate wasmparser;
use inkwell::basic_block::{self, BasicBlock};
use inkwell::module::{self, Linkage};
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, PointerType};
use inkwell::values::{ArrayValue, BasicMetadataValueEnum, BasicValue, GlobalValue, PointerValue};
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue};
use inkwell::{builder, context, AddressSpace, IntPredicate};
use inkwell::{builder::Builder, context::Context, module::Module};
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::unix::process;
use std::path::Path;
use std::rc::Rc;
use wasmparser::{
    BinaryReader, BlockType, CodeSectionReader, FunctionBody, FunctionSectionReader, Global,
    Operator, OperatorsReader, Parser, Payload, StructuralType,
};

// ************************REGISTER BANK************************

#[derive(Copy, Clone, Debug)]
enum Value<'a> {
    IntVar(IntValue<'a>),
    FloatVar(FloatValue<'a>),
    Function(FunctionValue<'a>),
    Global(inkwell::values::GlobalValue<'a>),
    Basic(BasicValueEnum<'a>),
    I32Const(i32),
}

#[derive(Debug)]

struct CustomStruct<'a> {
    builder: Builder<'a>,
    basic_block: Option<BasicBlock<'a>>,
    int_type: i32,
    fn_value: FunctionValue<'a>,
    function_counter: i32,
    return_nb: usize,
    regiser_number: usize,
}
#[derive(Copy, Clone, Debug)]
struct FunTypes {
    type_nb: usize,
    params_nb: usize,
    results_nb: usize,
}
#[derive(Copy, Clone, Debug)]
struct BBStruct<'a> {
    basic_block: BasicBlock<'a>,
    loop_block: usize,
}

struct Constructors<'a> {
    builer: Builder<'a>,
    context: Context,
    module: Module<'a>,
}

#[derive(Debug)]
struct ActualBlocks<'a> {
    //builder: Builder<'a>,
    basic_block: BasicBlock<'a>,
    function: FunctionValue<'a>,
}
#[derive(Debug)]
struct Register<'a> {
    value: Value<'a>,
}

impl<'a> Register<'a> {
    fn new(value: Value<'a>) -> Register<'a> {
        Register { value }
    }

    fn get_value(&self) -> &Value<'a> {
        &self.value
    }
}

// ************************ MAIN ************************

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::create();
    let module = context.create_module("hello-translation");
    let wasm_bytes = std::fs::read("src/lib/write_std_opti.wasm").expect("Unable to read wasm file");
    inkwell::targets::Target::initialize_all(&Default::default());
    // Parse the Wasm module
    // Iterate through the functions in the module
    let mut global_counter = 0;
    let mut function_counter = 0;
    let mut function_map: HashMap<String, CustomStruct> = HashMap::new();

    let parser = Parser::new(0);
    //let mut fun_types: Vec<FuncType> = Vec::new();
    let mut fun_types: HashMap<u32, FunTypes> = HashMap::new();
    let mut import_names: Vec<String> = Vec::new();
    let mut functions_parsed: Vec<u32> = Vec::new();
    let mut imports_parsed: Vec<u32> = Vec::new();
    let mut bodies: Vec<FunctionBody> = Vec::new();
    let mut globals: Vec<Global> = Vec::new();
    let i8_type = context.i8_type();
    let mut global_values_arr = vec![i8_type.const_zero(); 8*64 * 1024];
    //let mut global_values_arr = vec![i8_type.const_zero();1024];

    let mut memory_val: GlobalValue =
        module.add_global(context.i8_type().array_type(0), None, "my_global_var");

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
            Ok(Payload::MemorySection(_memories)) => {
                let memories = _memories.into_iter();
                let mut initial_size = 0;
                for size in memories {
                    initial_size = size.unwrap().initial;
                    println!("Memory size: {}", initial_size);
                }
                let page_size = 8*64 * 1024;
                //let page_size = 64 * 1024;
                let total_memory_size_bytes = initial_size as usize * page_size;
                println!("Memory size: {}", total_memory_size_bytes);
                let i8_type = context.i8_type();
                //let array_type = i8_type.array_type(total_memory_size_bytes as u32 );
                //for testing purposes
                let array_type = i8_type.array_type(page_size as u32);
                memory_val = module.add_global(array_type, None, "memory");
                memory_val.set_constant(false);
                memory_val.set_linkage(Linkage::External);
            }
            Ok(Payload::TypeSection(_types)) => {
                // Handle the type section here
                //TODO look if into_iter is okay
                //types.extend(_types.into_iter().collect::<Result<Vec<_>, _>>()?);
                for (i, fun_type) in _types.into_iter().enumerate() {
                    for param_type in fun_type.unwrap().types() {
                        //println!("    {:?}", param_type);
                        let val: &[wasmparser::ValType];
                        let val2: &[wasmparser::ValType];
                        match &param_type.structural_type {
                            StructuralType::Func(func_type) => {
                                // You have access to the FuncType variant here
                                // You can use func_type as needed
                                val = func_type.params();
                                val2 = func_type.results();

                                fun_types.insert(
                                    i as u32,
                                    FunTypes {
                                        type_nb: i,
                                        params_nb: val.len(),
                                        results_nb: val2.len(),
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(Payload::ImportSection(_imports)) => {
                // Handle the import section here
                for import in _imports {
                    let import = import?;
                    println!(
                        "  Import {}::{} %F{} Function Type: {:?}",
                        import.module, import.name, function_counter, import.ty
                    );
                    import_names.push(import.name.to_string());
                    match import.ty {
                        wasmparser::TypeRef::Func(_func_type) => {
                            imports_parsed.push(_func_type);
                        }

                        _ => {}
                    }
                }
            }
            Ok(Payload::FunctionSection(functions)) => {
                // Handle the function section here
                //TODO look if into_iter is okay

                functions_parsed.extend(functions.into_iter().collect::<Result<Vec<_>, _>>()?);
            }

            Ok(Payload::CodeSectionEntry(body)) => {
                // Handle the function body here
                let mut local_var = body.get_locals_reader().unwrap();
                let nb_local = local_var.get_count();
                if nb_local > 0 {
                    let local = local_var.read();
                    println!("  Local {:?}", local);
                }
                bodies.push(body);
            }

            Ok(Payload::GlobalSection(_globals)) => {
                // Handle the global section here
                globals.extend(_globals.into_iter().collect::<Result<Vec<_>, _>>()?);
            }

            Ok(Payload::DataSection(_data)) => {
                // Handle the data section here
                for item in _data {
                    let item = item?;
                    println!("  Data {:?}", item.data);
                    if let wasmparser::DataKind::Active {
                        memory_index,
                        offset_expr,
                    } = item.kind
                    {
                        for op in offset_expr.get_operators_reader() {
                            let op = op?;
                            println!("  Data {:?}", op);
                            match op {
                                Operator::I32Const { value } => {
                                    println!("i32.const {}", value);
                                    initialize_memory(
                                        &context,
                                        memory_val,
                                        &mut global_values_arr,
                                        item.data,
                                        value as u32,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    println!("----------------------IMPORTS-------------------");
    let imports_length = imports_parsed.len();
    for (index, imports) in imports_parsed.iter().enumerate() {
        let name = "%F".to_string() + &function_counter.to_string();
        println!("-----------------------------------------");
        println!("Import: {:?}", imports);
        let i32_type = context.i32_type();

        let type_nb = fun_types.get(&imports).unwrap().type_nb;
        let params_nb = fun_types.get(&imports).unwrap().params_nb;
        let return_nb = fun_types.get(&imports).unwrap().results_nb;

        let mut metadata_vec: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();
        for _ in 0..params_nb {
            metadata_vec.push(i32_type.into());
        }
        let fn_type: inkwell::types::FunctionType<'_>;

        if return_nb == 1 {
            fn_type = i32_type.fn_type(&metadata_vec, false);
        } else {
            fn_type = context.void_type().fn_type(&metadata_vec, false);
        }

        let fn_value = module.add_function(
            import_names.get(index).unwrap(),
            fn_type,
            Some(Linkage::DLLImport),
        );
        // let basic_block = context.append_basic_block(fn_value, "entry");
        let builder = context.create_builder();
        // builder.position_at_end(basic_block);

        function_map.insert(
            name.clone(),
            CustomStruct {
                builder: builder,
                basic_block: None,
                int_type: type_nb as i32,
                function_counter: function_counter,
                fn_value: fn_value,
                return_nb: return_nb,
                regiser_number: 0,
            },
        );

        function_counter += 1;
    }

    println!("----------------------FUNCTION TYPE-------------------");

    for (index, functions) in functions_parsed.iter().enumerate() {
        // Handle each function's operands here
        let name = "%F".to_string() + &function_counter.to_string();
        println!("-----------------------------------------");
        println!("Function name: {:?}", name);
        //TODO are there any more function types?
        let i32_type = context.i32_type();

        let type_nb = fun_types.get(&functions).unwrap().type_nb;
        let params_nb = fun_types.get(&functions).unwrap().params_nb;
        let return_nb = fun_types.get(&functions).unwrap().results_nb;

        let mut metadata_vec: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();
        for _ in 0..params_nb {
            metadata_vec.push(i32_type.into());
        }
        let fn_type: inkwell::types::FunctionType<'_>;

        if return_nb == 1 {
            fn_type = i32_type.fn_type(&metadata_vec, false);
        } else {
            fn_type = context.void_type().fn_type(&metadata_vec, false);
        }

        let fn_value = module.add_function(name.as_str(), fn_type, None);
        let basic_block = context.append_basic_block(fn_value, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);

        function_map.insert(
            name.clone(),
            CustomStruct {
                builder: builder,
                basic_block: Some(basic_block),
                int_type: type_nb as i32,
                function_counter: function_counter,
                fn_value: fn_value,
                return_nb: return_nb,
                regiser_number: 0,
            },
        );

        function_counter += 1;
    }

    println!("-------------------------GLOBAL SECTION------------------------------");
    println!("Global len: {:?}", globals.len());
    for global in globals {
        let module_ref = &module;

        println!("Global: {:?}", global);
        let val = global.init_expr.get_binary_reader();
        //let type = g.unwrap().ty;
        println!("Global type: {:?}", val);
        let name = format!("%G{}", global_counter);
        let global_value = module_ref.add_global(context.i32_type(), None, name.as_str());
        global_value.set_initializer(&context.i32_type().const_int(65592, false));
        global_value.set_constant(false);
        println!("Global: {:?}", name);
        global_counter += 1;
    }

    println!("-------------------------FUNCTION BODY------------------------------");
    let mut function_counter = imports_length as i32;
    for (index, body) in bodies.iter().enumerate() {
        let mut local_var = body.get_locals_reader().unwrap();
        let nb_local = local_var.get_count();
        if nb_local > 0 {
            let local = local_var.read();
            function_map
                .get_mut(&format!("%F{}", function_counter))
                .unwrap()
                .regiser_number = local.unwrap().0 as usize;
        }

        process_function_body(
            &body,
            &context,
            &module,
            &function_map,
            function_counter,
            memory_val,
            &mut global_values_arr,
        );
        function_counter += 1;
    }

    println!("-------------------------PRINT LLVM IR------------------------------");

    // Print LLVM IR code to the console
    println!("{}", module.print_to_string().to_string());

    module.verify().unwrap();
    module.write_bitcode_to_path(Path::new("hello_demo.bc"));
    println!("LLVM bitcode has been written to hello_demo.bc");

    let ir_string = module.print_to_string().to_string();
    let mut file = File::create("hello_demo.ll").expect("Failed to create file");
    file.write_all(ir_string.as_bytes())
        .expect("Failed to write to file");

    println!("LLVM IR has been written to hello_demo.ll");
    Ok(())
}

fn allocate_memory() {}
// //TODO Error handling?
// fn handle_function_type<'ctx>(
//     function: u32,
//     context: &'ctx Context,
//     module: &'ctx inkwell::module::Module<'ctx>,
//     function_counter: &mut i32,
//     function_map: &mut HashMap<String, CustomStruct<'ctx>>,
// ) {

// }

// ************************ HELPER FUNCTIONS ************************

fn process_function_body<'ctx>(
    body: &FunctionBody,
    context: &'ctx Context,
    module: &Module<'ctx>,
    function_map: &HashMap<String, CustomStruct<'ctx>>,
    fn_index: i32,
    memory_value: GlobalValue<'ctx>,
    global_values_arr: &mut Vec<IntValue<'ctx>>,
) {
    let mut code: OperatorsReader<'_> = body
        .get_operators_reader()
        .expect("Failed to get operators reader");

    let name = format!("%F{}", fn_index);
    println!("Function name: {}", name);
    let map_value = function_map.get(&name);

    match map_value {
        Some(value) => {
            let builder = &value.builder;
            let basic_block = value.basic_block.unwrap();
            let int_type = value.int_type;
            let function_counter = value.function_counter;
            let fn_value = value.fn_value;
            let return_nb = value.return_nb;
            let register_nb = value.regiser_number;
            builder.position_at_end(basic_block);
            process_function_body_helper(
                &mut code,
                context,
                module,
                builder,
                function_counter,
                basic_block,
                fn_value,
                function_map,
                return_nb,
                register_nb,
                memory_value,
                global_values_arr,
            );
        }
        None => {
            println!("Function not found");
        }
    }
}

fn initialize_memory<'ctx>(
    context: &'ctx Context,
    memory: GlobalValue<'ctx>,
    values: &mut Vec<IntValue<'ctx>>,
    data: &[u8],
    memory_index: u32,
) {
    let i8_type = context.i8_type();

    for (i, &byte) in data.iter().enumerate() {
        values[memory_index as usize + i] = i8_type.const_int(byte as u64, false);
    }
    let initializer = i8_type.const_array(&values);
    memory.set_initializer(&initializer);
}

// fn i32_to_i8s<'a>(context: &'a Context, value: IntValue<'a>) -> Vec<u8> {
//     let i8_type = context.i8_type();
//     let mut bytes = Vec::new();

//     for i in 0..4 {
//         // Shift the value right by i*8 bits and then truncate to i8
//         let shifted = value.const_ashr(i8_type.const_int(8 * i, false));
//         let byte = shifted.const_truncate(i8_type);

//         bytes.push(byte);
//     }

//     bytes
// }

fn i32_to_i8s(input: i32) -> Vec<u8> {
    let mut tmp = Vec::new();

    tmp.push((input & 0xff) as u8);
    tmp.push(((input >> 8) & 0xff) as u8);
    tmp.push(((input >> 16) & 0xff) as u8);
    tmp.push(((input >> 24) & 0xff) as u8);

    tmp
}

fn process_function_body_helper<'ctx>(
    code: &mut OperatorsReader<'_>,
    context: &'ctx Context,
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    function_counter: i32,
    entry_bb: BasicBlock<'ctx>,
    function: FunctionValue<'ctx>,
    function_map: &HashMap<String, CustomStruct>,
    return_nb: usize,
    register_nb: usize,
    memory_value: GlobalValue<'ctx>,
    global_values_arr: &mut Vec<IntValue<'ctx>>,
) {
    let mut stack: Vec<Value> = Vec::new();
    let mut current_bb = BBStruct {
        basic_block: entry_bb,
        loop_block: 0,
    };
    let mut pointer_value: PointerValue = memory_value.as_pointer_value();
    let mut bb_stack: Vec<BBStruct> = Vec::new();
    let mut current_block = BBStruct {
        basic_block: entry_bb,
        loop_block: 0,
    };
    let mut next = 0;
    let mut register_bank: Vec<PointerValue> = Vec::new();
    let mut prev_instruction_is_branch = false;

    for value in function.get_params() {
        let name = format!("%R{}_{}", next, function_counter);
        let err = builder.build_alloca(value.get_type(), &name);
        register_bank.push(err.unwrap());

        next += 1;
    }
    for _ in 0..register_nb {
        let name = format!("%R{}_{}", next, function_counter);
        let err = builder.build_alloca(context.i32_type(), &name);
        register_bank.push(err.unwrap());

        next += 1;
    }

    while !code.eof() {
        let val_to_read = code.read().unwrap();

        match val_to_read {
            Operator::I32Const { value } => {
                stack.push(Value::I32Const(value));
                println!("i32.const {}", value);
            }
            Operator::Call { function_index } => {
                let name = format!("%F{}", function_index);
                let called_function = function_map.get(&name);
                let nb_args = called_function.unwrap().fn_value.count_params();
                let mut args: Vec<BasicMetadataValueEnum> = Vec::new();
                stack.reverse();
                for _ in 0..nb_args {
                    let arg: Value<'_> = stack.pop().unwrap();
                    args.push(BasicMetadataValueEnum::IntValue(handle_value(arg, context)));
                }
                let ret_val = builder
                    .build_direct_call(called_function.unwrap().fn_value, &args, &name)
                    .unwrap()
                    .try_as_basic_value()
                    .left();

                match ret_val {
                    Some(val) => {
                        stack.push(Value::Basic(val));
                    }
                    None => {}
                }

                println!("call {}", function_index);
            }
            Operator::I32Add => {
                let rhs: Value<'_> = stack.pop().unwrap();
                let lhs: Value<'_> = stack.pop().unwrap();

                let int_value_rhs = handle_value(rhs, context);
                let int_value_lhs = handle_value(lhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_add(int_value_lhs, int_value_rhs, &name);
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.add");
            }
            Operator::I32Sub => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_rhs = handle_value(rhs, context);
                let int_value_lhs = handle_value(lhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_sub(int_value_lhs, int_value_rhs, &name);

                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.sub");
            }
            Operator::I32Mul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_lhs = handle_value(lhs, context);
                let int_value_rhs = handle_value(rhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_mul(int_value_lhs, int_value_rhs, &name);
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.mul")
            }

            Operator::I32DivU => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_lhs = handle_value(lhs, context);
                let int_value_rhs = handle_value(rhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_unsigned_div(int_value_lhs, int_value_rhs, &name);
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.div_u")
            }

            Operator::I32DivS => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_lhs = handle_value(lhs, context);
                let int_value_rhs = handle_value(rhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_signed_div(int_value_lhs, int_value_rhs, &name);
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.div_s")
            }

            Operator::GlobalSet { global_index } => {
                let name = format!("%G{}", global_index);
                let value = stack.pop().unwrap();

                let _global: inkwell::values::GlobalValue<'_> = module.get_global(&name).unwrap();
                let ptr = _global.as_pointer_value();
                //builder.position_at_end(entry_bb);
                let _ = builder.build_store(ptr, handle_value(value, context));
                println!("global.set {}", global_index);
            }
            Operator::GlobalGet { global_index } => {
                //builder.build_load(context.i32_type(), ptr, "%G1");
                let name = format!("%G{}", global_index);
                let _global: inkwell::values::GlobalValue<'_> = module.get_global(&name).unwrap();
                let ptr = _global.as_pointer_value();
                //builder.position_at_end(entry_bb);
                let value = builder.build_load(context.i32_type(), ptr, "%G0");

                //let value: inkwell::values::IntValue<'_> = _global.as_basic_value_enum().into_int_value();
                stack.push(Value::IntVar(value.unwrap().into_int_value()));
                println!("global.get {}", global_index);
            }

            Operator::LocalGet { local_index } => {
                let name = format!("%R{}_{}", next, function_counter);
                let pointer_val = register_bank.get(local_index as usize).unwrap();
                let value = builder.build_load(context.i32_type(), *pointer_val, &name);
                stack.push(Value::IntVar(value.unwrap().into_int_value()));

                println!("local.get {}", local_index);
            }

            Operator::LocalTee { local_index } => {
                let local_var = stack.pop().unwrap();
                let value_to_store = local_var.clone();
                stack.push(value_to_store);
                let name = format!("%R{}", local_index);
                let pointer_val = register_bank.get(local_index as usize).unwrap();
                let _ = builder.build_store(*pointer_val, handle_value(local_var, context));

                println!("local.tee {}", local_index);
            }

            Operator::LocalSet { local_index } => {
                let local_var = stack.pop().unwrap();
                let name = format!("%R{}", local_index);
                let pointer_val = register_bank.get(local_index as usize).unwrap();
                let _ = builder.build_store(*pointer_val, handle_value(local_var, context));

                println!("local.set {}", local_index);
            }

            Operator::End => {
                if bb_stack.len() == 0 {
                    if return_nb == 1 {
                        let value = stack.pop();
                        let _err = builder.build_return(Some(
                            &handle_value(value.unwrap(), context).as_basic_value_enum(),
                        ));
                    } else {
                        builder.build_return(None);
                    }
                    break;
                }

                let block = bb_stack.pop().unwrap();
                //println!("BB_STACK IN END: {:?}", bb_stack);
                //println!("BLOCK FROM STACK IN END: {:?}", block);
                if block.loop_block != 1 {
                    //println!("CURRENT BLOCK IN END: {:?}", current_block);
                    //println!(
                    //    "PREV INSTRUCTION IS BRANCH: {:?}",
                    //    prev_instruction_is_branch
                    //);
                    if (prev_instruction_is_branch == false) {
                        let instr = builder.build_unconditional_branch(block.basic_block);
                        //println!("INSTRUCTION IN END: {:?}", instr);
                    }
                    builder.position_at_end(block.basic_block);
                    current_block = BBStruct {
                        basic_block: block.basic_block,
                        loop_block: 0,
                    };
                    prev_instruction_is_branch = false;
                } else {
                    bb_stack.pop();
                }
                println!("end");
            }

            Operator::Loop { blockty } => {
                bb_stack.push(current_block);
                //println!("bb_stack in loop: {:?}", bb_stack);
                let block = context.append_basic_block(function, "loop");
                builder.build_unconditional_branch(block);
                builder.position_at_end(block);
                current_block = BBStruct {
                    basic_block: block,
                    loop_block: 1,
                };
                bb_stack.push(current_block);
                next += 1;

                println!("loop {:?}", blockty);
            }

            Operator::Block { blockty } => {
                let after_block = context.append_basic_block(function, "after_end");
                bb_stack.push(BBStruct {
                    basic_block: (after_block),
                    loop_block: (0),
                });
                //println!("bb_stack in block{:?}", bb_stack);
                // let block = context.append_basic_block(function, "block");
                // builder.position_at_end(block);
                // current_block = block;

                println!("block {:?}", blockty);
            }

            Operator::BrIf { relative_depth } => {
                let branch_block = bb_stack
                    .get(bb_stack.len() - 1 - (relative_depth as usize))
                    .unwrap();
                let continue_block = context.append_basic_block(function, "else");

                let value = stack.pop().unwrap();
                let int_var = handle_value(value, context);
                //cast every i32
                let cmp = builder.build_int_compare(
                    IntPredicate::NE,
                    int_var,
                    context.i32_type().const_int(0, false),
                    "cmpeq",
                );
                let _ = builder.build_conditional_branch(
                    cmp.unwrap(),
                    branch_block.basic_block,
                    continue_block,
                );
                builder.position_at_end(continue_block);
                current_block = BBStruct {
                    basic_block: continue_block,
                    loop_block: 0,
                };

                println!("br_if {}", relative_depth);
            }

            Operator::Br { relative_depth } => {
                let branch_block = bb_stack
                    .get(bb_stack.len() - 1 - (relative_depth as usize))
                    .unwrap();
                let _ = builder.build_unconditional_branch(branch_block.basic_block);
                prev_instruction_is_branch = true;
                println!("br {}", relative_depth);
            }

            Operator::If { blockty } => {
                // let condition = stack.pop().unwrap();
                // let val = handle_value(condition, context);

                // let then_block = context.append_basic_block(function, "then");
                // let else_block = context.append_basic_block(function, "else");
                // let merge_block = context.append_basic_block(function, "ifcont");

                // builder.build_conditional_branch(val, then_block, else_block);

                // // Populate then_block
                // builder.position_at_end(then_block);
                // // Pseudo-code: Add instructions for the 'then' sequence.
                // builder.build_unconditional_branch(else_block);

                // // Populate else_block, if there is one
                // builder.position_at_end(else_block);
                // // Pseudo-code: Add instructions for the 'else' sequence, if any.
                // builder.build_unconditional_branch(merge_block);

                // // Continue with merge_block
                // builder.position_at_end(merge_block);

                println!("if {:?}", blockty);
            }

            Operator::Return => {
                let value = stack.pop();
                match value {
                    Some(Value) => {
                        let int_var = handle_value(value.unwrap(), context);
                        builder.build_return(Some(&int_var));
                    }
                    None => {
                        builder.build_return(None);
                    }
                }

                println!("return");
            }
            Operator::I32GtU => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::UGT,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.gt_u");
            }
            Operator::I32LtU => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::ULT,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.lt_u");
            }
            Operator::I32LtS => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.lt_s");
            }
            Operator::I32LeU => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::ULE,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.le_u");
            }
            Operator::I32Eqz => {
                let value = stack.pop().unwrap();
                let int_var = handle_value(value, context);
                let zero = context.i32_type().const_int(0, false);
                let one = context.i32_type().const_int(1, false);
                let res = int_var.eq(&zero);
                match res {
                    true => {
                        stack.push(Value::IntVar(one));
                    }
                    false => {
                        stack.push(Value::IntVar(zero));
                    }
                }
                println!("i32.eqz");
            }
            Operator::I32Ne => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.ne");
            }
            Operator::I32Eq => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result = builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.eq");
            }
            // Operator::I32Store { memarg } => {
            //     let value = stack.pop().unwrap();
            //     let address = stack.pop().unwrap();
            //     let int_value_address = handle_value(address, context);
            //     let int_value_value = handle_value(value, context);
            //     let ptr_tmp = builder.build_ptr_to_int(pointer_value, context.i64_type(), "ptr");
    
            //     let int_value_address_cast = builder.build_int_z_extend(
            //         int_value_address,
            //         context.i64_type(),
            //         "cast",
            //     );
    
            //     let int_value_address_tmp = builder.build_int_add(
            //         int_value_address_cast.unwrap(),
            //         ptr_tmp.unwrap(),
            //         "add_ptr",
            //     );
    
            //     let final_ptr = builder.build_int_to_ptr(
            //         int_value_address_tmp.unwrap(),
            //         context.i64_type().ptr_type(AddressSpace::default()),
            //         "ptr_build",
            //     );
            //     println!("{} {} {}", int_value_address, int_value_value, memarg.offset);
            //     println!("{:?}", final_ptr);
                
            //     let _ = builder.build_store(final_ptr.unwrap(), int_value_value);
            //     println!("i32.store");
            // }
            Operator::I32Store { memarg } => {
                let value = stack.pop().unwrap();
                let base_address = stack.pop().unwrap();

                let intval_base_address = handle_value(base_address, context);
                let intval_value = handle_value(value, context);
                let ptr_tmp = builder.build_ptr_to_int(pointer_value, context.i64_type(), "ptr");

                
                let offset = context.i64_type().const_int(memarg.offset, false);

                let intval_base_address_cast = builder.build_int_z_extend(
                    intval_base_address,
                    context.i64_type(),
                    "cast",
                );

                let offset_tmp = builder.build_int_add(offset, intval_base_address_cast.unwrap(), "add_offset");

                let int_value_address_tmp = builder.build_int_add(
                    offset_tmp.unwrap(),
                    ptr_tmp.unwrap(),
                    "add_ptr",
                );


                let final_ptr = builder.build_int_to_ptr(
                    int_value_address_tmp.unwrap(),
                    context.i64_type().ptr_type(AddressSpace::default()),
                    "ptr_build",
                );
                println!("{} {} {}", intval_base_address, intval_value, offset);
                println!("{:?}", final_ptr);


                let _ = builder.build_store(final_ptr.unwrap(), intval_value);
                println!("i32.store: {:?}", memarg);
            }

            Operator::I32Load { memarg } => {
                let address = stack.pop().unwrap();
                let int_value_address = handle_value(address, context);
                let ptr_tmp = builder.build_ptr_to_int(pointer_value, context.i64_type(), "ptr");

                let int_value_address_cast = builder.build_int_z_extend(
                    int_value_address,
                    context.i64_type(),
                    "cast",
                );

                let int_value_address_tmp = builder.build_int_add(
                    int_value_address_cast.unwrap(),
                    ptr_tmp.unwrap(),
                    "add_ptr",
                );

                let final_ptr = builder.build_int_to_ptr(
                    int_value_address_tmp.unwrap(),
                    context.i64_type().ptr_type(AddressSpace::default()),
                    "ptr_build",
                );

                let ptr = final_ptr.unwrap();

                let value = builder.build_load(context.i64_type(),ptr, "load");
                stack.push(Value::IntVar(value.unwrap().into_int_value()));
                println!("i32.load");
            }

            Operator::I32RemU => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_lhs = handle_value(lhs, context);
                let int_value_rhs = handle_value(rhs, context);
                let name = format!("%{}", next);
                let result = builder.build_int_unsigned_rem(int_value_lhs, int_value_rhs, &name);
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.rem_u")
            }

            Operator::Drop => {
                stack.pop();
                println!("drop");
            }

            Operator::Select => {
                let false_cond = stack.pop().unwrap();
                let true_cond = stack.pop().unwrap();
                let cond = stack.pop().unwrap();

                let int_value_cond = handle_value(cond, context);
                let int_value_lhs = handle_value(true_cond, context);
                let int_value_rhs = handle_value(false_cond, context);

                let result = builder.build_select(
                    int_value_cond,
                    int_value_lhs,
                    int_value_rhs,
                    next.to_string().as_str(),
                );

                stack.push(Value::IntVar(result.unwrap().into_int_value()));
                next += 1;
                println!("select");
            }
            // Handle other operators as needed
            _ => {
                // Ignore unhandled operators for simplicity
                println!("Unhandled operator: {:?}", val_to_read);
            }
        }
    }
}

fn handle_value<'a>(rhs: Value<'a>, context: &'a Context) -> IntValue<'a> {
    let int_value_rhs = match rhs {
        Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
        Value::I32Const(int_const) => context.i32_type().const_int(int_const as u64, false),
        Value::Global(global_var) => global_var.as_basic_value_enum().into_int_value(),
        Value::Basic(basic_value) => match basic_value {
            BasicValueEnum::IntValue(int_value) => int_value,
            _ => {
                // Handle other cases or provide a default value if necessary
                panic!("Value cannot be transformed into IntMathValue");
            }
        },
        _ => {
            // Handle other cases or provide a default value if necessary
            panic!("Value cannot be transformed into IntMathValue");
        }
    };

    int_value_rhs
}

fn export_function(function: FunctionValue) {
    function.set_linkage(Linkage::External);
}

// fn map_block_type_to_llvm(block_type: BlockType, context: &Context) -> BasicTypeEnum {
//     match block_type {
//         BlockType::Empty => inkwell::types::AnyTypeEnum::VoidType(context.void_type()),
//         BlockType::Type(wasmparser::ValType::I32) => {
//             inkwell::types::AnyTypeEnum::IntType(context.i32_type())
//         }
//         // ... handle other Wasm types ...
//         _ => unimplemented!(),
//     }
// }
