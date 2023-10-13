extern crate inkwell;

use inkwell::context::Context;
use inkwell::targets::{Target, TargetMachine};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, IntValue, BasicValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize LLVM and create a context
    let context = Context::create();
    let module = context.create_module("mulAdd");
    let i32_type = context.i32_type();

    // Declare the multiplyAndAdd function
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into(), i32_type.into()], false);
    let function = module.add_function("multiplyAndAdd", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    // Function body for multiplyAndAdd
    let a = function.get_nth_param(0).unwrap().into_int_value();
    let b = function.get_nth_param(1).unwrap().into_int_value();
    let c = function.get_nth_param(2).unwrap().into_int_value();
    let result = builder.build_int_add(builder.build_int_mul(a, b, "mult").unwrap(), c, "sum");
    builder.build_return(Some(&(result.unwrap())));

    // Define the main function
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", main_fn_type, None);
    let main_block = context.append_basic_block(main_function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(main_block);

    // Call multiplyAndAdd function
    let num1 = context.i32_type().const_int(5, false);
    let num2 = context.i32_type().const_int(3, false);
    let num3 = context.i32_type().const_int(2, false);
    let result = builder.build_call(function, &[num1.into(), num2.into(), num3.into()], "result").unwrap()
    .try_as_basic_value()
    .left()
    .unwrap().into_pointer_value();
    let result = builder.build_load(i32_type,result, "result");
    const MY_CONST_STRING: &[u8] = b"Result: %d\n";


    let message = context.const_string(MY_CONST_STRING, false);
    let message_global = module.add_global(message.get_type(), None, "message");
    message_global.set_initializer(&message);
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::from(0));
    let message_ptr = builder.build_pointer_cast(
        message_global.as_pointer_value(),
        i8_ptr_type,
        "message_ptr",
    );


    builder.build_call(
        module.get_function("printf").unwrap(),
        &[BasicMetadataValueEnum::PointerValue(message_ptr.unwrap()), BasicMetadataValueEnum::IntValue((BasicValueEnum::into_int_value(result.unwrap())))],
        "printf",
    );
    builder.build_return(Some(&context.i32_type().const_int(0, false)));

    // Print LLVM IR to console
    println!("{}", module.print_to_string().to_string());

    // Write the LLVM IR to a file
    let ir_string = module.print_to_string().to_string();
    let mut file = File::create("output.ll").expect("Failed to create file");
    file.write_all(ir_string.as_bytes())
        .expect("Failed to write to file");

    println!("LLVM IR has been written to output.ll");

    Ok(())
}
