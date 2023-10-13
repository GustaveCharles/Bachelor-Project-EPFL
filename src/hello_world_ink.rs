extern crate inkwell;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::FunctionType;
use inkwell::values::{FunctionValue, IntValue, BasicMetadataValueEnum};
use inkwell::OptimizationLevel;
use std::fs::File;
use std::io::Write;

fn main() {
    // Create a context and a module
    let context = Context::create();
    let module = context.create_module("hello");

    // Define the main function signature: i32 ()*
    let function_type: FunctionType = context.void_type().fn_type(&[], false);

    // Create the main function
    let function: FunctionValue = module.add_function("main", function_type, None);

    // Define a basic block in the function
    let basic_block = context.append_basic_block(function, "entry");

    // Position the builder at the start of the basic block
    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    // Create the "Hello, World!" string constant
    const MY_CONST_STRING: &[u8] = b"Hello, World!";

    let hello_world_str = context.const_string(MY_CONST_STRING, false);
    let hello_world_global = module.add_global(
        hello_world_str.get_type(),
        None,
        "message",
    );
    builder.build_store(hello_world_global.as_pointer_value(), hello_world_str);

    // Call the `puts` function from the C standard library to print the string
    //inkwell::AddressSpace::from(0) means that the address space is the generic (=default) address space
    let puts_function_type: FunctionType = context.i32_type().fn_type(
        &[context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::from(0))
            .into()],
        false,
    );
    let puts_function: FunctionValue = module.add_function("puts", puts_function_type, None);
    let res = builder.build_call(
        puts_function,
        &[BasicMetadataValueEnum::ArrayValue(hello_world_str)],
        "puts",
    );

    // Return void from the main function
    builder.build_return(None);

    
    module.verify().unwrap();

    let ir_string = module.print_to_string().to_string();
    // Print the LLVM IR to the console
    println!("{}", ir_string);

    // Write the LLVM IR to a file
    let mut file = File::create("output.ll").expect("Failed to create file");
    file.write_all(ir_string.as_bytes()).expect("Failed to write to file");

    println!("LLVM IR has been written to output.ll");
}
//
