extern crate inkwell;

use inkwell::context::Context;
use inkwell::targets::{Target, TargetMachine};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, IntValue};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize LLVM and create a context
    let context = Context::create();
    let module = context.create_module("hello");

    // Define the global variable @message with the string value
    const MY_CONST_STRING: &[u8] = b"Hello, World!";
    let message = context.const_string(MY_CONST_STRING, false);
    let message_global = module.add_global(message.get_type(), None, "message");
    message_global.set_initializer(&message);
    //message_global.set_linkage(inkwell::module::Linkage::Internal);

    // Declare the external puts function
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::from(0));
    let puts_type = context.i32_type().fn_type(&[i8_ptr_type.into()], false);
    let puts = module.add_function("puts", puts_type, None);
    //puts.set_linkage(inkwell::module::Linkage::External);

    // Define the main function
    let fn_type = context.void_type().fn_type(&[], false);
    let main_func = module.add_function("main", fn_type, None);
    let entry_bb = context.append_basic_block(main_func, "entry");

    // Set up the builder and position it at the end of the entry block
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    // Call puts function with the global string as an argument
    let message_ptr = builder.build_pointer_cast(
        message_global.as_pointer_value(),
        i8_ptr_type,
        "message_ptr",
    );
    builder.build_call(
        puts,
        &[BasicMetadataValueEnum::PointerValue(message_ptr.unwrap())],
        "puts_call",
    );

    // Return void from main
    builder.build_return(None);

    // Print LLVM IR code to the console
    println!("{}", module.print_to_string().to_string());

    // Optionally, generate an object file using llc and clang
    // let target_triple = TargetMachine::get_default_triple();
    // let target = Target::from_triple(&target_triple)?;
    // let target_machine = target.create_target_machine(&target_triple, "", "", Target::OptimizationLevel::Default)?;

    // target_machine.write_to_file(&module, inkwell::targets::FileType::Object, "output.o")?;

    // Write the LLVM IR to a file
    module.verify().unwrap();
    let ir_string = module.print_to_string().to_string();
    let mut file = File::create("hello_works_test_ink.ll").expect("Failed to create file");
    file.write_all(ir_string.as_bytes())
        .expect("Failed to write to file");

    println!("LLVM IR has been written to hello_works_test.ll");

    Ok(())
}
