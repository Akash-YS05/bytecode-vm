#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    //arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    Push,

    //variables
    StoreVar,
    LoadVar,

    //comparison operators
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,

    //control flow
    Jump,   //unconditional jump
    JumpIfFalse,    //jump if T.O.S is falsy

    //program termination
    Halt
}

impl OpCode {
    pub fn convert_from_u8(byte: u8) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::Add),
            1 => Some(OpCode::Sub),
            2 => Some(OpCode::Mul),
            3 => Some(OpCode::Div),
            4 => Some(OpCode::Push),
            5 => Some(OpCode::Halt),
            6 => Some(OpCode::StoreVar),
            7 => Some(OpCode::LoadVar),
            _ => None,
        }
    }
    pub fn convert_to_u8(self) -> u8 {
        match self {
            OpCode::Add => 0,
            OpCode::Sub => 1,
            OpCode::Mul => 2,
            OpCode::Div => 3,
            OpCode::Push => 4,
            OpCode::Halt => 5,
            OpCode::StoreVar => 6,
            OpCode::LoadVar => 7,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_conversion() {
        assert_eq!(OpCode::convert_from_u8(0), Some(OpCode::Add));
        assert_eq!(OpCode::convert_from_u8(5), Some(OpCode::Halt));
        assert_eq!(OpCode::convert_from_u8(50), None);

        assert_eq!(OpCode::Add.convert_to_u8(), 0);
        assert_eq!(OpCode::Push.convert_to_u8(), 4);
    }
}