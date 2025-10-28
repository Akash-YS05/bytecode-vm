use crate::vm::VM;
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string};

pub fn fibonacci_implementation() {
    println!("Fibonacci Implementation Example (f0 = 0, f1 = 1)");

    let mut vm = VM::new();
    let mut bytecode: Vec<u8> = vec![];

    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(0));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("f0"));

    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(1));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("f1"));

    // Compute f2 = f0 + f1
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("f0"));
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("f1"));
    bytecode.push(OpCode::Add.convert_to_u8());

    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("f2"));

    bytecode.push(OpCode::Halt.convert_to_u8());

    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            println!("f0 = {:?}", vm.get_variable("f0"));
            println!("f1 = {:?}", vm.get_variable("f1"));
            println!("f2 = {:?}", vm.get_variable("f2"));
        }
        Err(e) => println!("Error: {}", e),
    }


}
