extern crate wasmparser;
use inkwell::basic_block::BasicBlock;
use inkwell::module;
use inkwell::values::{BasicMetadataValueEnum, BasicValue};
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue};
use inkwell::{builder::Builder, context::Context, module::Module};
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::unix::process;
use std::rc::Rc;
use wasmparser::{
    BinaryReader, CodeSectionReader, FunctionBody, FunctionSectionReader, Global, Operator,
    OperatorsReader, Parser, Payload, StructuralType,
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

struct CustomStruct<'a> {
    builder: Builder<'a>,
    basic_block: BasicBlock<'a>,
    int_type: i32,
    fn_value: FunctionValue<'a>,
}

struct FunTypes {
    type_nb: usize,
    params_nb: usize,
    results_nb: usize,
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

// Define the RegisterBank struct
#[derive(Debug)]
struct RegisterBank<'a> {
    registers: HashMap<String, Register<'a>>,
}

impl<'a> RegisterBank<'a> {
    fn new() -> RegisterBank<'a> {
        RegisterBank {
            registers: HashMap::new(),
        }
    }
    //registers will be 0 initialized
    fn create_register(&mut self, name: &str, value: Value<'a>) {
        // Create a new register with the provided value and insert it into the HashMap
        self.registers
            .insert(name.to_string(), Register::new(value));
    }

    fn read_register(&self, name: &str) -> Option<&Value<'a>> {
        // Read the value of a register by name
        self.registers.get(name).map(|reg| reg.get_value())
    }

    fn write_register(&mut self, name: &str, value: Value<'a>) -> bool {
        // Write a value to a register by name
        if let Some(register) = self.registers.get_mut(name) {
            register.value = value;
            println!("REGISTER WRITE TRUE");
            true
        } else {
            println!("REGISTER WRITE FALSE");
            false
        }
    }
}

// ************************ MAIN ************************

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::create();
    let module = context.create_module("branches_opti_shorter-translation");
    let wasm_bytes =
        std::fs::read("src/lib/simplest_branch_nonOpti.wasm").expect("Unable to read wasm file");

    // Parse the Wasm module
    // Iterate through the functions in the module
    let mut global_counter = 0;
    let mut function_counter = 0;
    let mut function_map: HashMap<String, CustomStruct> = HashMap::new();

    let parser = Parser::new(0);
    //let mut fun_types: Vec<FuncType> = Vec::new();
    let mut fun_types: HashMap<u32, FunTypes> = HashMap::new();
    let mut functions_parsed: Vec<u32> = Vec::new();
    let mut bodies: Vec<FunctionBody> = Vec::new();
    let mut globals: Vec<Global> = Vec::new();

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
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
                //imports.extend(_imports.into_iter().collect::<Result<Vec<_>, _>>()?);
            }
            Ok(Payload::FunctionSection(functions)) => {
                // Handle the function section here
                //TODO look if into_iter is okay
                functions_parsed.extend(functions.into_iter().collect::<Result<Vec<_>, _>>()?);
            }

            Ok(Payload::CodeSectionEntry(body)) => {
                // Handle the function body here
                bodies.push(body);
            }

            Ok(Payload::GlobalSection(_globals)) => {
                // Handle the global section here
                globals.extend(_globals.into_iter().collect::<Result<Vec<_>, _>>()?);
            }
            _ => {}
        }
    }

    println!("----------------------FUNCTION TYPE-------------------");
    for functions in functions_parsed {
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
                basic_block: basic_block,
                int_type: type_nb as i32,
                fn_value: fn_value,
            },
        );

        function_counter += 1;
    }

    println!("-------------------------GLOBAL SECTION------------------------------");
    println!("Global len: {:?}", globals.len());
    for global in globals {
        let module_ref = &module;

        println!("Global: {:?}", global);
        //let type = g.unwrap().ty;
        let name = format!("%G{}", global_counter);
        let value = module_ref.add_global(
            context.i32_type(),
            Some(inkwell::AddressSpace::from(0)),
            name.as_str(),
        );
        println!("Global: {:?}", name);
        global_counter += 1;
    }

    println!("-------------------------FUNCTION BODY------------------------------");
    let mut function_counter = 0;
    for body in bodies {
        println!("Function body instructions:");

        process_function_body(&body, &context, &module, &function_map, function_counter);
        function_counter += 1;
    }

    println!("-------------------------PRINT LLVM IR------------------------------");

    // Print LLVM IR code to the console
    println!("{}", module.print_to_string().to_string());

    module.verify().unwrap();
    let ir_string = module.print_to_string().to_string();
    let mut file = File::create("hello_works.ll").expect("Failed to create file");
    file.write_all(ir_string.as_bytes())
        .expect("Failed to write to file");

    println!("LLVM IR has been written to hello_works.ll");
    Ok(())
}

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

fn process_function_body(
    body: &FunctionBody,
    context: &Context,
    module: &Module,
    function_map: &HashMap<String, CustomStruct>,
    fn_index: i32,
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
            let basic_block = value.basic_block;
            let int_type = value.int_type;
            println!("Function type: {}", int_type);
            let fn_value = value.fn_value;
            builder.position_at_end(basic_block);
            process_function_body_helper(
                &mut code,
                context,
                module,
                builder,
                basic_block,
                fn_value,
                function_map,
            );
        }
        None => {
            println!("Function not found");
        }
    }
}

fn process_function_body_helper(
    code: &mut OperatorsReader<'_>,
    context: &Context,
    module: &Module<'_>,
    builder: &Builder<'_>,
    entry_bb: BasicBlock,
    function: FunctionValue<'_>,
    function_map: &HashMap<String, CustomStruct>,
) {
    let mut stack: Vec<Value> = Vec::new();
    let mut bb_stack: Vec<BasicBlock> = Vec::new();
    let mut current_block = entry_bb;
    let mut next = 0;

    let mut register_bank = RegisterBank::new();
    let parameters = function.get_params();
    let value1 = context.i32_type().const_int(5, false);
    register_bank.create_register("%R0", Value::IntVar(value1));
    for value in parameters {
        let name = format!("%R{}", next);
        let value2 = Value::IntVar(value.into_int_value());
        register_bank.create_register(&name, value2);
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
                //TODO régler le problème de l'index de la fonction appelée et ce sera bon je pense
                // let name = format!("%F{}", function_index-2);
                // let called_function = function_map.get(&name);
                // let nb_args = called_function.unwrap().fn_value.count_params();
                // let mut args: Vec<BasicMetadataValueEnum> = Vec::new();
                // for _ in 0..nb_args {
                //     let arg: Value<'_> = stack.pop().unwrap();
                //     args.push(BasicMetadataValueEnum::IntValue((handle_value(arg, context))));
                // }
                // builder.build_call(called_function.unwrap().fn_value, &args, &name);
                println!("call {}", function_index);
            }
            Operator::I32Add => {
                //println!("stack: {:?}", stack);
                let rhs: Value<'_> = stack.pop().unwrap();
                let lhs: Value<'_> = stack.pop().unwrap();

                let int_value_rhs = handle_value(rhs, context);
                let int_value_lhs = handle_value(lhs, context);

                let result =
                    builder.build_int_add(int_value_lhs, int_value_rhs, next.to_string().as_str());
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.add");
            }
            Operator::I32Sub => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_rhs = handle_value(rhs, context);
                let int_value_lhs = handle_value(lhs, context);

                let result =
                    builder.build_int_sub(int_value_lhs, int_value_rhs, next.to_string().as_str());

                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.sub");
            }
            Operator::I32Mul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_lhs = handle_value(lhs, context);
                let int_value_rhs = handle_value(rhs, context);

                let result =
                    builder.build_int_mul(int_value_lhs, int_value_rhs, next.to_string().as_str());
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.mul")
            }
            Operator::GlobalSet { global_index } => {
                let name = format!("%G{}", global_index);
                let value = stack.pop().unwrap();

                let _global: inkwell::values::GlobalValue<'_> = module.get_global(&name).unwrap();
                let ptr = _global.as_pointer_value();
                builder.position_at_end(entry_bb);
                let _ = builder.build_store(ptr, handle_value(value, context));
                println!("global.set {}", global_index);
            }
            Operator::GlobalGet { global_index } => {
                //builder.build_load(context.i32_type(), ptr, "%G1");
                let name = format!("%G{}", global_index);
                let _global: inkwell::values::GlobalValue<'_> = module.get_global(&name).unwrap();
                let ptr = _global.as_pointer_value();
                builder.position_at_end(entry_bb);
                let value = builder.build_load(context.i32_type(), ptr, "%G0");

                //let value: inkwell::values::IntValue<'_> = _global.as_basic_value_enum().into_int_value();
                stack.push(Value::IntVar(value.unwrap().into_int_value()));
                println!("global.get {}", global_index);
            }

            Operator::LocalGet { local_index } => {
                //TODO no corresponding %1 value
                //(i32.const 23)
                //(local.set 0)
                //(local.get 0)
                //%0          = 23    ; (i32.const 23)  / local_versions = [nil,nil]
                //%local.0.v0 = %0    ; (local.set 0)   / local_versions = [  0,nil]
                //%1 = %local.0.v0    ; (local.get 0)   / local_versions = [  0,nil]
                let name = format!("%R{}", local_index);
                let register_val = register_bank.read_register(&name);
                let register_val_cloned = register_val.clone();

                stack.push(*register_val_cloned.unwrap());

                println!("local.get {}", local_index);
            }

            Operator::LocalTee { local_index } => {
                let local_var = stack.pop().unwrap();
                let value_to_store = local_var.clone();
                stack.push(value_to_store);
                let name = format!("%R{}", local_index);
                register_bank.write_register(&name, local_var);

                println!("local.tee {}", local_index);
            }

            Operator::LocalSet { local_index } => {
                let local_var = stack.pop().unwrap();
                let name = format!("%R{}", local_index);
                register_bank.write_register(&name, local_var);

                println!("local.set {}", local_index);
            }

            Operator::End => {
                if bb_stack.len() == 0 {
                    break;
                }
                let block = bb_stack.pop().unwrap();
                builder.build_unconditional_branch(block);
                builder.position_at_end(block);
                current_block = block;

                println!("end");
            }

            Operator::Block { blockty } => {
                bb_stack.push(current_block);
                let block = context.append_basic_block(function, "block");
                builder.position_at_end(block);
                current_block = block;

                println!("block {:?}", blockty);
            }

            Operator::BrIf { relative_depth } => {
                println!("bb_stack: {:?}", bb_stack);
                let branch_block = bb_stack
                    .get(bb_stack.len() - 1 - (relative_depth as usize))
                    .unwrap();
                let continue_block = context.append_basic_block(function, "else");

                let value = stack.pop().unwrap();
                let int_var = handle_value(value, context);
                let _ = builder.build_conditional_branch(int_var, *branch_block, continue_block);
                builder.position_at_end(continue_block);
                current_block = continue_block;

                println!("br_if {}", relative_depth);
            }

            Operator::If { blockty } => {
                let condition = stack.pop().unwrap();
                let val = match condition {
                    Value::IntVar(int_var) => int_var,
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntVar");
                    }
                };

                let then_block = context.append_basic_block(function, "then");
                let else_block = context.append_basic_block(function, "else");
                let merge_block = context.append_basic_block(function, "ifcont");

                builder.build_conditional_branch(val, then_block, else_block);

                // Populate then_block
                builder.position_at_end(then_block);
                // Pseudo-code: Add instructions for the 'then' sequence.
                builder.build_unconditional_branch(else_block);

                // Populate else_block, if there is one
                builder.position_at_end(else_block);
                // Pseudo-code: Add instructions for the 'else' sequence, if any.
                builder.build_unconditional_branch(merge_block);

                // Continue with merge_block
                builder.position_at_end(merge_block);

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

                let result =
                    builder.build_int_compare(inkwell::IntPredicate::UGT, int_value_lhs, int_value_rhs, next.to_string().as_str());
                
                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.gt_u");
            }
            Operator::I32LtU => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let int_value_lhs = handle_value(left, context);
                let int_value_rhs = handle_value(right, context);

                let result =
                    builder.build_int_compare(inkwell::IntPredicate::ULT, int_value_lhs, int_value_rhs, next.to_string().as_str());
                
                stack.push(Value::IntVar(result.unwrap()));

                println!("i32.lt_u");
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
        _ => {
            // Handle other cases or provide a default value if necessary
            panic!("Value cannot be transformed into IntMathValue");
        }
    };

    int_value_rhs
}
