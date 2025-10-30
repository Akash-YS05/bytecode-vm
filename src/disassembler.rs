use crate::{opcode::OpCode};

// a disassembler converts bytecode into readable instruction   
pub struct Disassembler {
    bytecode: Vec<u8>,
    offset: usize,
}

impl Disassembler {
    pub fn new(bytecode: Vec<u8>) -> Self {
        Disassembler { bytecode, offset: 0 }
    }

    fn read_byte_solution(&mut self) -> Option<u8> {
        if self.offset >= self.bytecode.len() {
            return None;
        }

        let byte = self.bytecode[self.offset];
        self.offset += 1;
        Some(byte)
    }

    fn read_i64_solution(&mut self) -> Option<i64> {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_byte_solution()?;

        }

        Some(i64::from_le_bytes(bytes))
    }

    fn read_usize_solution(&mut self) -> Option<usize> {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_byte_solution()?;
        }
        Some(usize::from_le_bytes(bytes))
    }

    fn read_string_solution(&mut self) -> Option<String> {
        let len = self.read_byte_solution()? as usize;
        let mut bytes = Vec::with_capacity(len);
        
        for _ in 0..len {
            bytes.push(self.read_byte_solution()?);
        }

        String::from_utf8(bytes).ok()
    }

    fn disassemble_instruction(&mut self) -> Option<String> {
        let start_offset = self.offset;
        let byte = self.read_byte_solution()?;
        let opcode = OpCode::convert_from_u8(byte)?;
        
        let instruction = match opcode {
            OpCode::Push => {
                let value = self.read_i64_solution()?;
                format!("{:04} PUSH {}", start_offset, value)
            }
            
            OpCode::StoreVar | OpCode::LoadVar | 
            OpCode::StoreLocal | OpCode::LoadLocal | OpCode::Print => {
                let name = self.read_string_solution()?;
                format!("{:04} {} \"{}\"", start_offset, opcode.name(), name)
            }
            
            OpCode::Jump | OpCode::JumpIfFalse | OpCode::Call => {
                let addr = self.read_usize_solution()?;
                format!("{:04} {} {}", start_offset, opcode.name(), addr)
            }
            
            _ => {
                format!("{:04} {}", start_offset, opcode.name())
            }
        };
        
        Some(instruction)
    }
    
    // disassemble all bytecode
    pub fn disassemble(&mut self) -> String {
        let mut output = String::new();
        output.push_str("Bytecode Disassembly:\n");
        output.push_str("ADDR INSTRUCTION\n");
        output.push_str("---- -----------\n");
        
        while self.offset < self.bytecode.len() {
            if let Some(instruction) = self.disassemble_instruction() {
                output.push_str(&instruction);
                output.push('\n');
            } else {
                output.push_str(&format!("{:04} <invalid>\n", self.offset));
                break;
            }
        }
        
        output
    }
}

pub fn disassemble(bytecode: Vec<u8>) -> String {
    let mut disas= Disassembler::new(bytecode);
    disas.disassemble()
}


