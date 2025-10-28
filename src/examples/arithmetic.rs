use crate::vm::VM;
use crate::opcode::OpCode;

pub fn example_addition() {
    println!("Example 1: Simple Addition (10 + 5)");
    
    let mut vm = VM::new();
    
    // Manually construct bytecode
    let bytecode = vec![
        OpCode::Push.convert_to_u8(), 10,
        OpCode::Push.convert_to_u8(), 5,
        OpCode::Add.convert_to_u8(),
        OpCode::Halt.convert_to_u8(),
    ];
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            if let Some(result) = vm.peek_stack() {
                println!("Result: {:?}", result);
            }
        }
        Err(e) => println!("Error: {}", e.display_solution()),
    }
    
    println!();
}

pub fn example_complex() {
    println!("Example 2: Complex Expression ((20 / 4) * 3) - 2");
    
    let mut vm = VM::new();
    
    // bytecode for the expression
    let bytecode = vec![
        OpCode::Push.convert_to_u8(), 20,
        OpCode::Push.convert_to_u8(), 4,
        OpCode::Div.convert_to_u8(),      // Stack: [5]
        OpCode::Push.convert_to_u8(), 3,
        OpCode::Mul.convert_to_u8(),      // Stack: [15]
        OpCode::Push.convert_to_u8(), 2,
        OpCode::Sub.convert_to_u8(),      // Stack: [13]
        OpCode::Halt.convert_to_u8(),
    ];
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            if let Some(result) = vm.peek_stack() {
                println!("Result: {:?}", result);
                println!("Full stack: {:?}", vm.get_stack());
            }
        }
        Err(e) => println!("Error: {}", e.display_solution()),
    }
    
    println!();
}

// pub fn homework_example_solution() {
//     println!("HOMEWORK Example: (15 - 5) * 2 + 10");
    
//     let mut vm = VM::new();
    
//     let bytecode = vec![
//         OpCode::Push.convert_to_u8(), 15,
//         OpCode::Push.convert_to_u8(), 5,
//         OpCode::Sub.convert_to_u8(),      // Stack: [10]
//         OpCode::Push.convert_to_u8(), 2,
//         OpCode::Mul.convert_to_u8(),      // Stack: [20]
//         OpCode::Push.convert_to_u8(), 10,
//         OpCode::Add.convert_to_u8(),      // Stack: [30]
//         OpCode::Halt.convert_to_u8(),
//     ];
    
//     vm.load_bytecode_solution(bytecode);
    
//     match vm.run_solution() {
//         Ok(_) => {
//             if let Some(result) = vm.peek_stack() {
//                 println!("Result: {:?}", result);
//                 println!("Expected: Integer(30)");
//                 assert_eq!(result, value::Value::Integer(30));
//             }
//         }
//         Err(e) => println!("Error: {}", e.display_solution()),
//     }
    
//     println!();
// }

