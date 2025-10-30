use crate::{disassembler::disassemble, vm::VM};
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string, encode_usize};

pub fn example_hello_world() {
    println!("Example 1: Hello World with Print");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    bytecode.push(OpCode::Print.convert_to_u8());
    bytecode.extend(encode_string("Hello, "));
    
    bytecode.push(OpCode::Print.convert_to_u8());
    bytecode.extend(encode_string("World!"));
    
    bytecode.push(OpCode::PrintLn.convert_to_u8());
    
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    vm.load_bytecode_solution(bytecode);
    vm.run_solution().unwrap();
    
    println!();
}

pub fn example_factorial_with_print() {
    println!("Example 2: Factorial(5) with Output");
    println!("Computing 5! = 5 * 4 * 3 * 2 * 1");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    // result = 1
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("result"));
    
    // n = 5
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(5));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    
    // loop: while n > 0
    let loop_start = bytecode.len();
    
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::Gt.convert_to_u8());
    
    bytecode.push(OpCode::JumpIfFalse.convert_to_u8());
    let loop_exit_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0));
    
    // result = result * n
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("result"));
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    bytecode.push(OpCode::Mul.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("result"));
    
    // n = n - 1
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::Sub.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("n"));
    
    // jump back to loop start
    bytecode.push(OpCode::Jump.convert_to_u8());
    bytecode.extend(encode_usize(loop_start));
    
    // loop end
    let loop_end = bytecode.len();
    
    // print result
    bytecode.push(OpCode::Print.convert_to_u8());
    bytecode.extend(encode_string("5! = "));
    
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("result"));
    bytecode.push(OpCode::PrintVal.convert_to_u8());
    
    bytecode.push(OpCode::PrintLn.convert_to_u8());
    
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // fix loop exit jump
    let end_bytes = encode_usize(loop_end);
    for (i, &byte) in end_bytes.iter().enumerate() {
        bytecode[loop_exit_placeholder + i] = byte;
    }
    
    vm.load_bytecode_solution(bytecode);
    vm.run_solution().unwrap();
    
    println!();
}

pub fn example_disassembler() {
    println!("Example 3: Bytecode Disassembler");
    
    let mut bytecode = vec![];
    
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(42));
    
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("x"));
    
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("x"));
    
    bytecode.push(OpCode::PrintVal.convert_to_u8());
    
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // disassemble the bytecode
    let disassembly = disassemble(bytecode.clone());
    println!("{}", disassembly);
    
    println!("Execution output: ");
    let mut vm = VM::new();
    vm.load_bytecode_solution(bytecode);
    vm.run_solution().unwrap();
    println!("\n");
}
