extern crate wasmparser;
use inkwell::basic_block::BasicBlock;
use inkwell::module;
use inkwell::values::BasicValue;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue};
use inkwell::{builder::Builder, context::Context, module::Module};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Ref;
use std::os::unix::process;
use wasmparser::{
    BinaryReader, CodeSectionReader, FunctionBody, FunctionSectionReader, Global, Operator,
    OperatorsReader, Parser, Payload,
};

// ************************REGISTER BANK************************

#[derive(Copy, Clone)]
enum Value<'a> {
    IntVar(IntValue<'a>),
    FloatVar(FloatValue<'a>),
    Function(FunctionValue<'a>),
    Global(inkwell::values::GlobalValue<'a>),
    Basic(BasicValueEnum<'a>),
    I32Const(i32),
}
// struct Register {
//     value: Value<'static>,
// }

// impl Register {
//     fn new() -> Register {
//         Register { value: Value::I32Const(0) }
//     }

//     fn read(&self) -> Value<'_> {
//         self.value
//     }

//     fn write(&mut self, data: Value<'_>) {
//         self.value = data;
//     }
// }


// struct RegisterBank {
//     registers: Vec<RefCell<Register>>, // Use RefCell for interior mutability
// }

// impl RegisterBank {
//     fn new(num_registers: usize) -> RegisterBank {
//         let mut registers = Vec::with_capacity(num_registers);
//         for _ in 0..num_registers {
//             registers.push(RefCell::new(Register::new()));
//         }
//         RegisterBank { registers }
//     }

//     fn read_register(&self, index: usize) -> Value<'_> {
//         self.registers[index].borrow().read()
//     }

//     fn write_register(&mut self, index: usize, data: Value<'_>) {
//         self.registers[index].borrow_mut().write(data);
//     }
// }


struct Register {
    value: Value<'static>, // Assuming 'static lifetime here, adjust as needed
}

impl Register {
    fn new() -> Register {
        Register {
            value: Value::I32Const(0), // Initialize with a default value if necessary
        }
    }

    fn read(&self) -> &Value {
        &self.value
    }

    fn write(&mut self, data: Value<'static>) {
        self.value = data;
    }
}

struct RegisterBank {
    registers: Vec<RefCell<Register>>,
}

impl RegisterBank {
    fn new(num_registers: usize) -> RegisterBank {
        let mut registers = Vec::with_capacity(num_registers);
        for _ in 0..num_registers {
            registers.push(RefCell::new(Register::new()));
        }
        RegisterBank { registers }
    }

    fn read_register(&self, index: usize) -> Ref<'_, Value<'static>>  {
        Ref::map(self.registers[index].borrow(), |r| r.read())
    }

    fn write_register(&self, index: usize, data: Value<'static>) {
        self.registers[index].borrow_mut().write(data);
    }
}


struct CustomStruct<'a> {
    builder: Builder<'a>,
    basic_block: BasicBlock<'a>,
    int_type: i32,
    fn_value: FunctionValue<'a>,
}

// ************************ MAIN ************************

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::create();
    let module = context.create_module("hello-translation");
    let wasm_bytes =
        std::fs::read("src/lib/write_std_opti.wasm").expect("Unable to read wasm file");

    // Parse the Wasm module
    // Iterate through the functions in the module
    let mut global_counter = 0;
    let mut function_counter = 0;
    let mut function_map: HashMap<String, CustomStruct> = HashMap::new();

    let parser = Parser::new(0);

    let mut functions_parsed: Vec<u32> = Vec::new();
    let mut bodies: Vec<FunctionBody> = Vec::new();
    let mut globals: Vec<Global> = Vec::new();

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
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
        println!("Function: {}", functions);
        println!("Function name: {}", name);

        //TODO are there any more function types?
        match functions {
            0 => {
                //TODO is name okay
                let fn_type: inkwell::types::FunctionType<'_> = context
                    .void_type()
                    .fn_type(&[context.i32_type().into()], false);
                let fn_value = module.add_function(name.as_str(), fn_type, None);
                let basic_block = context.append_basic_block(fn_value, "entry");
                let builder = context.create_builder();
                builder.position_at_end(basic_block);
                let value: IntValue<'_> = fn_value.get_first_param().unwrap().into_int_value();
                let target_type = context.i32_type().ptr_type(inkwell::AddressSpace::from(0));
                let pointer_value = builder.build_int_to_ptr(
                    value,       // the integer value
                    target_type, // the target pointer type
                    "inttoptr",  // name for the generated instruction
                );

                builder.build_store(pointer_value.unwrap(), value);
                //println!("Error: {:?}", err);

                function_map.insert(
                    name.clone(),
                    CustomStruct {
                        builder: builder,
                        basic_block: basic_block,
                        int_type: 0,
                        fn_value: fn_value,
                    },
                );
                //builder.build_return(None);

                function_counter += 1;
            }
            1 => {
                let fn_type = context.void_type().fn_type(&[], false);
                let fn_value = module.add_function(name.as_str(), fn_type, None);
                let basic_block = context.append_basic_block(fn_value, "entry");
                let builder = context.create_builder();
                builder.position_at_end(basic_block);

                function_map.insert(
                    name.clone(),
                    CustomStruct {
                        builder: builder,
                        basic_block: basic_block,
                        int_type: 1,
                        fn_value: fn_value,
                    },
                );

                function_counter += 1;
            }
            2 => {
                println!("Function type not implemented yet");
                function_counter += 1;
            }
            3 => {
                println!("Function type not implemented yet");
                function_counter += 1;
            }
            _ => {
                println!("Function type not supported");
                function_counter += 1;
            }
        }

        // handle_function_type(
        //     functions,
        //     &context,
        //     &module,
        //     &mut function_counter,
        //     &mut function_map,
        // );
    }

    println!("-------------------------GLOBAL SECTION------------------------------");
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

    for body in bodies {
        println!("Function body instructions:");
        process_function_body(&body, &context, &module, &function_map);
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
) {
    let mut code: OperatorsReader<'_> = body
        .get_operators_reader()
        .expect("Failed to get operators reader");

    let map_value = function_map.get("%F1");



    match map_value {
        Some(value) => {
            let builder = &value.builder;
            let basic_block = value.basic_block;
            let int_type = value.int_type;
            let fn_value = value.fn_value;
            println!("Int type: {}", int_type);
            builder.position_at_end(basic_block);
            process_function_body_helper(
                &mut code,
                context,
                module,
                builder,
                basic_block,
                fn_value
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
    module: &Module,
    builder: &Builder,
    entry_bb: BasicBlock,
    function: FunctionValue<'_>
) {
    let mut stack: Vec<Value> = Vec::new();
    let mut next = 0;

    let num_registers = 10;
    let register_bank = Rc::new(RefCell::new(RegisterBank::new(num_registers))); // Use Rc<RefCell<RegisterBank>>

    //let fn_value = module.add_function(...);

    while !code.eof() {
        match code.read().unwrap() {
            Operator::I32Const { value } => {
                stack.push(Value::I32Const(value));
                println!("i32.const {}", value);
            }
            Operator::Call { function_index } => {
                println!("call {}", function_index);
            }
            Operator::I32Add => {
                //println!("stack: {:?}", stack);
                let rhs: Value<'_> = stack.pop().unwrap();
                let lhs: Value<'_> = stack.pop().unwrap();

                let int_value_rhs = match rhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    Value::I32Const(int_const) => {
                        context.i32_type().const_int(int_const as u64, false)
                    }
                    Value::Global(global_var) => global_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let int_value_lhs = match lhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    Value::I32Const(int_const) => {
                        context.i32_type().const_int(int_const as u64, false)
                    }
                    Value::Global(global_var) => global_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let result =
                    builder.build_int_add(int_value_lhs, int_value_rhs, next.to_string().as_str());
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.add");
            }
            Operator::I32Sub => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let int_value_rhs = match rhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    Value::I32Const(int_const) => {
                        context.i32_type().const_int(int_const as u64, false)
                    }
                    Value::Global(global_var) => global_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let int_value_lhs = match lhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    Value::I32Const(int_const) => {
                        context.i32_type().const_int(int_const as u64, false)
                    }
                    Value::Global(global_var) => global_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let result =
                    builder.build_int_sub(int_value_lhs, int_value_rhs, next.to_string().as_str());

                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.sub");
            }
            Operator::I32Mul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                let int_value_rhs = match rhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let int_value_lhs = match lhs {
                    Value::IntVar(int_var) => int_var.as_basic_value_enum().into_int_value(),
                    _ => {
                        // Handle other cases or provide a default value if necessary
                        panic!("Value cannot be transformed into IntMathValue");
                    }
                };
                let result =
                    builder.build_int_mul(int_value_lhs, int_value_rhs, next.to_string().as_str());
                stack.push(Value::IntVar(result.unwrap()));
                next += 1;
                println!("i32.mul")
            }
            Operator::GlobalGet { global_index } => {
                //builder.build_load(context.i32_type(), ptr, "%G1");
                let _global: inkwell::values::GlobalValue<'_> = module.get_global("%G0").unwrap();
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
                let register_val = register_bank.borrow().read_register(local_index as usize);
                let register_val_cloned = register_val.clone();

                stack.push(register_val_cloned);

                println!("local.get {}", local_index);
            }

            Operator::LocalTee { local_index } => {
                // Create a local variable of type i32
                //let local_var = function.add_alloca(context.i32_type(), None, "my_local_var");
                let local_var = stack.pop().unwrap();
                let value_to_store = local_var.clone();
                stack.push(value_to_store);

                register_bank.borrow().write_register(local_index as usize, local_var);       

                // Set the local variable to a value (e.g., 42)
                //let value_to_store = context.i32_type().const_int(42, false);
                //builder.build_store(local_var, value_to_store);

                // Use local.tee-like operation to both set the local variable and return its value
                //let loaded_value = builder
                //   .build_load(context.i32_type(),local_var, "loaded_value");

                // Now, 'loaded_value' contains the same value as the local variable and can be used as the result
                //builder.build_return(loaded_value.unwrap().as_basic_value_enum());
                println!("local.tee {}", local_index);
            }

            Operator::End => {
                println!("end");
            }

            Operator::Block { blockty } => {
                println!("block {:?}", blockty);
            }

            Operator::BrIf { relative_depth } => {
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

            // Handle other operators as needed
            _ => {
                // Ignore unhandled operators for simplicity
                println!("Unhandled operator: {:?}", code.read().unwrap());
            }
        }

    }
}
