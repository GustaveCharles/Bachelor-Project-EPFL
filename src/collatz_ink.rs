extern crate inkwell;

use inkwell::context::Context;
use inkwell::types::IntType;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::builder::Builder;

fn main() {
    let context = Context::create();
    let module = context.create_module("collatz_conjecture");
    let i32_type = context.i32_type();

    // Declare collatz_conjecture function
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let collatz_function = module.add_function("collatz_conjecture", fn_type, None);
    let collatz_entry_block = context.append_basic_block(&collatz_function, "entry");
    let collatz_builder = context.create_builder();
    collatz_builder.position_at_end(&collatz_entry_block);

    let n_param = collatz_function.get_nth_param(0).unwrap().into_int_value();
    let printf_format_str = context.const_string("%d ", false);
    let printf_format_str_ptr = collatz_builder.build_global_string_ptr("printf_format_str", &printf_format_str);
    collatz_builder.build_call(
        module.get_function("printf").unwrap(),
        &[printf_format_str_ptr.into(), n_param.into()],
        "printf",
    );

    let loop_block = context.append_basic_block(&collatz_function, "loop");
    let end_block = context.append_basic_block(&collatz_function, "end");

    collatz_builder.build_conditional_branch(collatz_builder.build_int_compare(inkwell::values::IntPredicate::EQ, n_param, i32_type.const_int(1, false), "compare"), &end_block, &loop_block);

    collatz_builder.position_at_end(&loop_block);
    let is_even = collatz_builder.build_int_compare(inkwell::values::IntPredicate::EQ, collatz_builder.build_int_signed_div(n_param, i32_type.const_int(2, false), "div_result"), i32_type.const_int(0, false), "is_even");
    let then_block = context.append_basic_block(&collatz_function, "then");
    let else_block = context.append_basic_block(&collatz_function, "else");
    collatz_builder.build_conditional_branch(is_even, &then_block, &else_block);

    collatz_builder.position_at_end(&then_block);
    let n_divided_by_2 = collatz_builder.build_int_signed_div(n_param, i32_type.const_int(2, false), "n_divided_by_2");
    collatz_builder.build_call(collatz_function, &[n_divided_by_2.into()], "");
    collatz_builder.build_unconditional_branch(&end_block);

    collatz_builder.position_at_end(&else_block);
    let n_times_3_plus_1 = collatz_builder.build_int_add(collatz_builder.build_int_mul(n_param, i32_type.const_int(3, false), ""), i32_type.const_int(1, false), "n_times_3_plus_1");
    collatz_builder.build_call(collatz_function, &[n_times_3_plus_1.into()], "");
    collatz_builder.build_unconditional_branch(&end_block);

    collatz_builder.position_at_end(&end_block);
    collatz_builder.build_return(None);

    // Define the main function
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", main_fn_type, None);
    let main_entry_block = context.append_basic_block(&main_function, "entry");
    let main_builder = context.create_builder();
    main_builder.position_at_end(&main_entry_block);

    let printf_input_str = context.const_string("Enter a positive integer: ", false);
    let printf_input_str_ptr = main_builder.build_global_string_ptr("printf_input_str", &printf_input_str);
    main_builder.build_call(module.get_function("printf").unwrap(), &[printf_input_str_ptr.into()], "");

    let scanf_format_str = context.const_string("%d", false);
    let num_ptr = main_builder.build_alloca(i32_type, "num_ptr");
    main_builder.build_call(
        module.get_function("scanf").unwrap(),
        &[scanf_format_str.into(), num_ptr.into()],
        "scanf_call",
    );

    let num = main_builder.build_load(num_ptr, "num");

    let invalid_input_block = context.append_basic_block(&main_function, "invalid_input");
    let valid_input_block = context.append_basic_block(&main_function, "valid_input");

    main_builder.build_conditional_branch(main_builder.build_int_compare(inkwell::values::IntPredicate::SLE, num, i32_type.const_int(0, false), ""), &invalid_input_block, &valid_input_block);

    main_builder.position_at_end(&invalid_input_block);
    let invalid_input_str = context.const_string("Invalid input. Please enter a positive integer.\n", false);
    let invalid_input_str_ptr = main_builder.build_global_string_ptr("invalid_input_str", &invalid_input_str);
    main_builder.build_call(module.get_function("printf").unwrap(), &[invalid_input_str_ptr.into()], "");
    main_builder.build_return(i32_type.const_int(1, false));

    main_builder.position_at_end(&valid_input_block);
    let printf_collatz_str = context.const_string("Collatz conjecture sequence for %d: ", false);
    let printf_collatz_str_ptr = main_builder.build_global_string_ptr("printf_collatz_str", &printf_collatz_str);
    main_builder.build_call(module.get_function("printf").unwrap(), &[printf_collatz_str_ptr.into(), num.into()], "");
    main_builder.build_call(collatz_function, &[num.into()], "");
    main_builder.build_call(module.get_function("printf").unwrap(), &[context.const_string("\n", false).as_pointer_value().into()], "");
    main_builder.build_return(i32_type.const_int(0, false));

    // Print LLVM IR to console
    println!("{}", module.print_to_string().to_string());
}
