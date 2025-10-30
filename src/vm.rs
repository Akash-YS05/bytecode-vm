use crate::opcode::OpCode;
use crate::value::Value;
use crate::error::VMError;
use crate::memory::Memory;
use crate::callframe::CallFrame;
use std::io::{self, Write};

pub struct VM {
    // intermediate values are stored in this stack
    stack: Vec<Value>,

    // call stack for function calls
    call_stack: Vec<CallFrame>,

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
            call_stack: vec![CallFrame::new_solution(0)],
            bytecode: Vec::new(),
            ip: 0,
            running: false,
            memory: Memory::new_solution(),
        }
    }

    //load into stack
    pub fn load_bytecode_solution(&mut self, code: Vec<u8>) {
        self.stack.clear();
        self.call_stack = vec![CallFrame::new_solution(0)];
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

    fn read_usize_solution(&mut self) -> Result<usize, VMError> {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_byte()?;
        }
        Ok(usize::from_le_bytes(bytes))
    }

    fn current_frame_mut(&mut self) -> Result<&mut CallFrame, VMError> {
        self.call_stack.last_mut().ok_or(VMError::StackUnderflow)
    }

    fn current_frame(&self) -> Result<&CallFrame, VMError> {
        self.call_stack.last().ok_or(VMError::StackUnderflow)
    }

    fn print_stack_trace(&self) {
        eprintln!("\n=== Call Stack Trace ===");
        for (i, frame) in self.call_stack.iter().enumerate().rev() {
            eprintln!("  #{} at IP {}", i, frame.return_address());
        }
        eprintln!("  Current IP: {}", self.ip); 
    }
    
    // Execute a single instruction
    fn execute_instruction(&mut self) -> Result<(), VMError> {
        // 1. Read the next byte (this is the opcode)
        // 2. Convert it to an OpCode using OpCode::from_u8
        // 3. Match on the opcode and execute it:
        let start_ip = self.ip;
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
            OpCode::Gt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.gt_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Lt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.lt_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Gte => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.gte_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Lte => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.lte_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Eq => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.eq_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }
            OpCode::Neq => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = a.neq_solution(b)
                    .ok_or(VMError::InvalidOperand)?;
                self.push(result);
            }

            //read the address and set the ip to that address
            OpCode::Jump => {
                let address = self.read_usize_solution()?;
                if address >= self.bytecode.len() {
                    return Err(VMError::OutOfBounds);
                }
                self.ip = address;
            }

            //read address, pop a value, jump to address if falsy or continue
            OpCode::JumpIfFalse => {
                let address = self.read_usize_solution()?;
                let condition = self.pop()?;

                if !condition.is_truthy_solution() {
                    if address >= self.bytecode.len() {
                        return Err(VMError::OutOfBounds);
                    }
                    self.ip = address;
                }
                //after this it'll just continue as ip has already advanced
             }
             OpCode::Call => {
                let function_address = self.read_usize_solution()?;
                let return_address = self.ip;
                let frame = CallFrame::new_solution(return_address);
                self.call_stack.push(frame);

                if function_address >= self.bytecode.len() {
                    return Err(VMError::OutOfBounds);
                }

                self.ip = function_address;
             }
             OpCode::Return => {
                if self.call_stack.len() <= 1 {
                    self.running = false;
                    return Ok(());
                }
                
                let frame = self.call_stack.pop().ok_or(VMError::StackUnderflow)?;
                self.ip = frame.return_address();
             }
             OpCode::StoreLocal => {
                let name = self.read_string_solution()?;
                let value = self.pop()?;
                self.current_frame_mut()?.store_local_solution(name, value);
            }
            
            OpCode::LoadLocal => {
                let name = self.read_string_solution()?;
                let value = self.current_frame()?
                    .load_local_solution(&name)
                    .ok_or_else(|| VMError::UndefinedVariable(name.clone()))?;
                self.push(value);
            }
            OpCode::Print => {
                let text = self.read_string_solution()?;
                print!("{}", text);
                io::stdout().flush().ok();
            }
            OpCode::PrintVal => {
                let value = self.pop()?;
                let output = match value {
                    Value::Integer(n) => n.to_string(),
                    Value::Boolean(b) => b.to_string(),
                };
                print!("{}", output);
                io::stdout().flush().ok();
            }
            
            OpCode::PrintLn => {
                println!();
            }
            OpCode::Halt => {
                self.running = false;
            }
            
            _ => return Err(VMError::InvalidOpCode(opcode.convert_to_u8())),

        }
        Ok(())
        
    }

    
    pub fn run_solution(&mut self) -> Result<(), VMError> {
        self.running = true;

        let max_instructions = 100_00;
        let mut instruction_count = 0;
        
        while self.running {
            instruction_count += 1;
            if instruction_count > max_instructions {
                return Err(VMError::InfiniteLoopDetected);
            }
            if let Err(e) = self.execute_instruction() {
                self.print_stack_trace();
                return Err(e);
            }
            // self.execute_instruction()?;
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

    pub fn current_ip(&self) -> usize {
        self.ip
    }

    pub fn call_stack_depth(&self) -> usize {
        self.call_stack.len()
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