use crate::vm::VM;
use crate::opcode::OpCode;
use super::utils::{encode_i64, encode_string};

pub fn example_simple_variable() {
    println!("Example: Simple Variable Storage and Retrieval");
    println!("Code: x=42");

    let mut vm = VM::new();
    let mut bytecode: Vec<u8> = vec![];

    // push 42
    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(42));

    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("x"));

    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("x"));

    bytecode.push(OpCode::Halt.convert_to_u8());

    vm.load_bytecode_solution(bytecode);

    match vm.run_solution() {
        Ok(_) => {
            println!("x = {:?}", vm.get_variable("x"));
            println!("Stack top: {:?}\n", vm.peek_stack());
        }
        Err(e) => println!("Error: {}\n", e),
    }

}

pub fn example_variable_arithmetic() {
    println!("Example: Variable Arithmetic");
    println!("Code: (a / b) * c where a=10 and b=2 and c=3");

    let mut vm = VM::new();
    let mut bytecode: Vec<u8> = vec![];

    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(10));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("a"));

    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(2));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("b"));

    bytecode.push(OpCode::Push.convert_to_u8());
    bytecode.extend(encode_i64(3));
    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("c"));

    //result: (a/b)*c
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("a"));
    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("b"));
    bytecode.push(OpCode::Div.convert_to_u8());

    bytecode.push(OpCode::LoadVar.convert_to_u8());
    bytecode.extend(encode_string("c"));
    bytecode.push(OpCode::Mul.convert_to_u8());

    bytecode.push(OpCode::StoreVar.convert_to_u8());
    bytecode.extend(encode_string("result"));

    bytecode.push(OpCode::Halt.convert_to_u8());

    vm.load_bytecode_solution(bytecode);

    match vm.run_solution() {
        Ok(_) => {
            println!("a = {:?}", vm.get_variable("a"));
            println!("b = {:?}", vm.get_variable("b"));
            println!("c = {:?}", vm.get_variable("c"));
            println!("result = {:?}", vm.get_variable("result"));
            println!("Expected: Integer(15)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }
    
}
