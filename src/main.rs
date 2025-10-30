mod opcode;
mod value;
mod error;
mod vm;
mod memory;
mod examples;
mod disassembler;
mod callframe;

use vm::VM;
use opcode::OpCode;

use examples::*;

fn main() {
    
    // variable::example_variable_arithmetic();
    // controls::example_simple_comparison();
    // controls::example_while_loop();

    printfn::example_hello_world();
    // printfn::example_factorial_with_print();
    printfn::example_disassembler();
    

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