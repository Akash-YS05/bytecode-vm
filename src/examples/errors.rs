use crate::vm::VM;
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string};

pub fn example_errors() {
    println!("Example 3: Error Handling");
    
    println!("  3a. Stack Underflow:");
    let mut vm = VM::new();
    let bytecode = vec![
        OpCode::Add.convert_to_u8(), 
        OpCode::Halt.convert_to_u8(),
    ];
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => println!("    Succeeded unexpectedly"),
        Err(e) => println!("    Caught error: {}", e.display_solution()),
    }
    
    println!("  3b. Division by Zero:");
    let mut vm = VM::new();
    let bytecode = vec![
        OpCode::Push.convert_to_u8(), 10,
        OpCode::Push.convert_to_u8(), 0,
        OpCode::Div.convert_to_u8(),
        OpCode::Halt.convert_to_u8(),
    ];
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => println!("    Succeeded unexpectedly"),
        Err(e) => println!("    Caught error: {}", e.display_solution()),
    }
    
    println!();
}