use crate::opcode::OpCode;
use crate::value::Value;
use crate::error::VMError;
use crate::memory::Memory;

pub struct VM {
    // intermediate values are stored in this stack
    stack: Vec<Value>,

    // the bytecode program (operands + instructions)
    bytecode: Vec<u8>,

    // instruction pointer to know which instruction is currently running
    ip: usize,

    //confirmation if VM is still running
    running: bool,

    //memory for storing variables
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            bytecode: Vec::new(),
            ip: 0,
            running: false,
            memory: Memory::new_solution(),
        }
    }

    //load into stack
    pub fn load_bytecode_solution(&mut self, code: Vec<u8>) {
        self.stack.clear();
        self.bytecode = code;
        self.ip = 0;
        self.running = false;
        self.memory.clear();
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    
    // Pop a value from the stack
    fn pop(&mut self) -> Result<Value, VMError> {

        //pop value from stack OR return StackUnderflow error
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }
    
    // Read the next byte from bytecode
    fn read_byte(&mut self) -> Result<u8, VMError> {

        // return OutOfBounds error if ip is outside bytecode length
        if self.ip >= self.bytecode.len() {
            return Err(VMError::OutOfBounds);
        }

        // read byte at current ip and increment ip
        let byte = self.bytecode[self.ip];
        self.ip += 1;

        Ok(byte)
    }

    fn read_string_solution(&mut self) -> Result<String, VMError> {

        // reading the length and that many bytes
        let length = self.read_byte()? as usize;
        let mut bytes = Vec::with_capacity(length);

        for _ in 0..length {
            bytes.push(self.read_byte()?);
        }

        String::from_utf8(bytes).map_err(|_| VMError::InvalidString)
    }

    fn read_i64_solution(&mut self) -> Result<i64, VMError> {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_byte()?;
        }
        Ok(i64::from_le_bytes(bytes))
    }
    
    // Execute a single instruction
    fn execute_instruction(&mut self) -> Result<(), VMError> {
        // 1. Read the next byte (this is the opcode)
        // 2. Convert it to an OpCode using OpCode::from_u8
        // 3. Match on the opcode and execute it:

        let byte = self.read_byte()?;
        let opcode = OpCode::convert_from_u8(byte).ok_or(VMError::InvalidOpCode(byte))?;

        match opcode {
            OpCode::Add => {
                let b  = self.pop()?;
                let a  = self.pop()?;

                let res = a.add_solution(b).ok_or(VMError::InvalidOperand)?;
                self.push(res);
            }
            OpCode::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.sub_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.mul_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.div_solution(b)
                    .ok_or(VMError::DivisionByZero)?;
                self.push(result);
            }
            OpCode::Push => {
                // Read the next byte as the value to push
                let value_byte = self.read_i64_solution()?;
                self.push(Value::int_solution(value_byte as i64));
            }
            OpCode::StoreVar => {
                let name = self.read_string_solution()?;
                let value = self.pop()?;
                self.memory.store_solution(name, value);
            }
            OpCode::LoadVar => {
                let name = self.read_string_solution()?;
                let value = self.memory.load_solution(&name)?;
                self.push(value);
            }
            OpCode::Halt => {
                self.running = false;
            }
        }
        Ok(())
        
    }
    
    pub fn run_solution(&mut self) -> Result<(), VMError> {
        self.running = true;
        
        while self.running {
            self.execute_instruction()?;
        }
        
        Ok(())
    }
    
    // Helper for testing: peek at top of stack without removing
    pub fn peek_stack(&self) -> Option<Value> {
        self.stack.last().copied()
    }
    
    // Helper for testing: get the entire stack
    pub fn get_stack(&self) -> &[Value] {
        &self.stack
    }

    // helper function
    pub fn get_variable(&self, name: &str) -> Result<Value, VMError> {
        self.memory.load_solution(name)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_addition() {
        let mut vm = VM::new();
        
        // Bytecode for: PUSH 10, PUSH 5, ADD, HALT
        let bytecode = vec![
            0, 10,  // PUSH 10
            0, 5,   // PUSH 5
            1,      // ADD
            5,      // HALT
        ];
        
        vm.load_bytecode_solution(bytecode);
        vm.run_solution().unwrap();
        
        assert_eq!(vm.peek_stack(), Some(Value::Integer(15)));
    }
    
    #[test]
    fn test_complex_calculation() {
        let mut vm = VM::new();
        
        // Bytecode for: PUSH 20, PUSH 4, DIV, PUSH 3, MUL, HALT
        // Should compute: (20 / 4) * 3 = 15
        let bytecode = vec![
            0, 20,  // PUSH 20
            0, 4,   // PUSH 4
            4,      // DIV
            0, 3,   // PUSH 3
            3,      // MUL
            5,      // HALT
        ];
        
        vm.load_bytecode_solution(bytecode);
        vm.run_solution().unwrap();
        
        assert_eq!(vm.peek_stack(), Some(Value::Integer(15)));
    }
    
    #[test]
    fn test_stack_underflow() {
        let mut vm = VM::new();
        
        // Try to ADD without pushing values first
        let bytecode = vec![1, 5]; // ADD, HALT
        
        vm.load_bytecode_solution(bytecode);
        let result = vm.run_solution();
        
        assert_eq!(result, Err(VMError::StackUnderflow));
    }
    
    #[test]
    fn test_division_by_zero() {
        let mut vm = VM::new();
        
        // PUSH 10, PUSH 0, DIV, HALT
        let bytecode = vec![0, 10, 0, 0, 4, 5];
        
        vm.load_bytecode_solution(bytecode);
        let result = vm.run_solution();
        
        assert_eq!(result, Err(VMError::DivisionByZero));
    }
}