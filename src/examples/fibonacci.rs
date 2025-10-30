use crate::{examples::utils::encode_usize, vm::VM};
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string};

pub fn example_fibonacci() {
    println!("Example 3: Fibonacci Sequence (first 10 numbers)");
    
    let mut vm = VM::new();
    let mut bytecode = vec![];
    
    // a = 0, b = 1, counter = 0
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("a"));
    
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("b"));
    
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    
    // print header
    bytecode.push(OpCode::Print.convert_to_u8());
    bytecode.extend(encode_string("Fibonacci: "));
    
    // loop: while counter < 10
    let loop_start = bytecode.len();
    
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(10));
    bytecode.push(OpCode::Lt.convert_to_u8());
    
    bytecode.push(OpCode::JumpIfFalse.convert_to_u8());
    let loop_exit_placeholder = bytecode.len();
    bytecode.extend(encode_usize(0));
    
    // print a
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("a"));
    bytecode.push(OpCode::PrintVal.convert_to_u8());
    
    bytecode.push(OpCode::Print.convert_to_u8());
    bytecode.extend(encode_string(" "));
    
    // temp = a + b
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("a"));
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("b"));
    bytecode.push(OpCode::Add.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("temp"));
    
    // a = b
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("b"));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("a"));
    
    // b = temp
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("temp"));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("b"));
    
    // counter = counter + 1
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::Add.convert_to_u8());
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("counter"));
    
    // jump back
    bytecode.push(OpCode::Jump.convert_to_u8());
    bytecode.extend(encode_usize(loop_start));
    
    // loop end
    let loop_end = bytecode.len();
    
    bytecode.push(OpCode::PrintLn.convert_to_u8());
    bytecode.push(OpCode::Halt.convert_to_u8());
    
    // fix jump
    let end_bytes = encode_usize(loop_end);
    for (i, &byte) in end_bytes.iter().enumerate() {
        bytecode[loop_exit_placeholder + i] = byte;
    }
    
    vm.load_bytecode_solution(bytecode);
    vm.run_solution().unwrap();
    
    println!();
}