use crate::vm::VM;
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string, encode_usize};

pub fn example_simple_comparison() {
    println!("Example 1: Simple Comparison");
    println!("Code: x = 10, y = 5, is_greater = (x > y)");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(10));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("x"));
    
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(5));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("y"));
    
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("x"));
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("y"));
    bytecode.push(OpCode::Gt.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("is_greater"));
    
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            println!("x = {:?}", vm.get_variable("x"));
            println!("y = {:?}", vm.get_variable("y"));
            println!("is_greater = {:?}", vm.get_variable("is_greater"));
            println!("Expected: Boolean(true)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }
}

pub fn example_if_else() {
    println!("Example 2: If/Else Statement");
    println!("Code:");
    println!("  age = 20");
    println!("  if age >= 18:");
    println!("      can_vote = 1");
    println!("  else:");
    println!("      can_vote = 0");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    // age = 20
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(20));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("age"));
    
    // age >= 18
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("age"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(18));
    bytecode.push(OpCode::Gte.convert_to_u8());
    
    // JUMP_IF_FALSE to else block
    let condition_end = bytecode.len();
    bytecode.push(OpCode::JumpIfFalse.convert_to_u8());
    
    // Reserve space for jump address 
    let jump_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0)); // Placeholder
    
    // Then block: can_vote = 1
    let then_start = bytecode.len();
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("can_vote"));
    
    // JUMP to end (skip else)
    bytecode.push(OpCode::Jump.convert_to_u8());
    let jump_to_end_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0)); // Placeholder
    
    // Else block: can_vote = 0
    let else_start = bytecode.len();
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("can_vote"));
    
    // End
    let end = bytecode.len();
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // Fix up jump addresses
    let else_bytes = encode_usize(else_start);
    for (i, &byte) in else_bytes.iter().enumerate() {
        bytecode[jump_placeholder + i] = byte;
    }
    
    let end_bytes = encode_usize(end);
    for (i, &byte) in end_bytes.iter().enumerate() {
        bytecode[jump_to_end_placeholder + i] = byte;
    }
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            println!("age = {:?}", vm.get_variable("age"));
            println!("can_vote = {:?}", vm.get_variable("can_vote"));
            println!("Expected: Integer(1)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }
}

pub fn example_while_loop() {
    println!("Example 3: While Loop");
    println!("Code:");
    println!("  counter = 0");
    println!("  while counter < 5:");
    println!("      counter = counter + 1");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    // counter = 0
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    
    // Loop start
    let loop_start = bytecode.len();
    
    // while counter < 5
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(5));
    bytecode.push(OpCode::Lt.convert_to_u8());
    
    // JUMP_IF_FALSE to end
    bytecode.push(OpCode::JumpIfFalse.convert_to_u8());
    let loop_exit_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0)); // Placeholder
    
    // Loop body: counter = counter + 1
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::Add.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    
    // Jump back to loop start
    bytecode.push(OpCode::Jump.convert_to_u8());
    bytecode.extend(encode_usize(loop_start));
    
    // Loop end
    let loop_end = bytecode.len();
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // Fix up exit jump
    let end_bytes = encode_usize(loop_end);
    for (i, &byte) in end_bytes.iter().enumerate() {
        bytecode[loop_exit_placeholder + i] = byte;
    }
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            println!("counter = {:?}", vm.get_variable("counter"));
            println!("Expected: Integer(5)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }
}

pub fn example_countdown() {
    println!("Example 4: Countdown Loop");
    println!("Code:");
    println!("  n = 10");
    println!("  while n > 0:");
    println!("      n = n - 1");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    // n = 10
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(10));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    
    // Loop start
    let loop_start = bytecode.len();
    
    // while n > 0
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::Gt.convert_to_u8());
    
    // JUMP_IF_FALSE to end
    bytecode.push(OpCode::JumpIfFalse.convert_to_u8());
    let loop_exit_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0));
    
    // Loop body: n = n - 1
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::Sub.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    
    // Jump back
    bytecode.push(OpCode::Jump.convert_to_u8());
    bytecode.extend(encode_usize(loop_start));
    
    // End
    let loop_end = bytecode.len();
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // Fix jump
    let end_bytes = encode_usize(loop_end);
    for (i, &byte) in end_bytes.iter().enumerate() {
        bytecode[loop_exit_placeholder + i] = byte;
    }
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            println!("n = {:?}", vm.get_variable("n"));
            println!("Expected: Integer(0)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }
}
