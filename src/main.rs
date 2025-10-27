mod opcode;
mod value;
mod error;
mod vm;
mod memory;

use vm::VM;
use opcode::OpCode;

fn main() {
    
    example_addition();
    
    example_complex();
    
    example_errors();

    example_with_builder();
    
}

fn example_addition() {
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

fn example_complex() {
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

fn example_errors() {
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

#[allow(dead_code)]
fn homework_example_solution() {
    println!("HOMEWORK Example: (15 - 5) * 2 + 10");
    
    let mut vm = VM::new();
    
    let bytecode = vec![
        OpCode::Push.convert_to_u8(), 15,
        OpCode::Push.convert_to_u8(), 5,
        OpCode::Sub.convert_to_u8(),      // Stack: [10]
        OpCode::Push.convert_to_u8(), 2,
        OpCode::Mul.convert_to_u8(),      // Stack: [20]
        OpCode::Push.convert_to_u8(), 10,
        OpCode::Add.convert_to_u8(),      // Stack: [30]
        OpCode::Halt.convert_to_u8(),
    ];
    
    vm.load_bytecode_solution(bytecode);
    
    match vm.run_solution() {
        Ok(_) => {
            if let Some(result) = vm.peek_stack() {
                println!("Result: {:?}", result);
                println!("Expected: Integer(30)");
                assert_eq!(result, value::Value::Integer(30));
            }
        }
        Err(e) => println!("Error: {}", e.display_solution()),
    }
    
    println!();
}

// while manually writing the opcodes and operands brings me back to comp architecture lectures, here is a builder api for ease

#[allow(dead_code)]
struct BytecodeBuilder {
    code: Vec<u8>,
}

impl BytecodeBuilder {
    fn new() -> Self {
        BytecodeBuilder { code: Vec::new()}
    }
    fn push_solution(mut self, value: i64) -> Self {
        self.code.push(OpCode::Push.convert_to_u8());
        self.code.push(value as u8);
        self
    }
    
    fn add_solution(mut self) -> Self {
        self.code.push(OpCode::Add.convert_to_u8());
        self
    }
    
    fn sub_solution(mut self) -> Self {
        self.code.push(OpCode::Sub.convert_to_u8());
        self
    }
    
    fn mul_solution(mut self) -> Self {
        self.code.push(OpCode::Mul.convert_to_u8());
        self
    }
    
    fn div_solution(mut self) -> Self {
        self.code.push(OpCode::Div.convert_to_u8());
        self
    }
    
    fn halt_solution(mut self) -> Self {
        self.code.push(OpCode::Halt.convert_to_u8());
        self
    }
    fn build(self) -> Vec<u8> {
        self.code
    }
}

#[allow(dead_code)]
fn example_with_builder() {
    let bytecode = BytecodeBuilder::new()
        .push_solution(10)
        .push_solution(5)
        .add_solution()
        .halt_solution()
        .build();
    
    let mut vm = VM::new();
    vm.load_bytecode_solution(bytecode);
    vm.run_solution().unwrap();
    
    println!("Builder result: {:?}", vm.peek_stack());
}