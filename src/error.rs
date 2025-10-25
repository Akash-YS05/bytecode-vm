use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum VMError {
    StackUnderflow,
    DivisionByZero,
    InvalidOpCode(u8),
    InvalidOperand,
    OutOfBounds,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self {
            VMError::StackUnderflow => {
                write!(f, "Stack underflow: tried to pop from empty stack")
            }
            VMError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            VMError::InvalidOpCode(byte) => {
                write!(f, "Invalid opcode: 0x{:02X} ({})", byte, byte)
            }
            VMError::InvalidOperand => {
                write!(f, "Invalid operand type for operation")
            }
            VMError::OutOfBounds => {
                write!(f, "Instruction pointer out of bounds")
            }
        }
    }
}

// This allows VMError to be used with the ? operator and Result types
impl std::error::Error for VMError {}

impl VMError {
    pub fn display_solution(&self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let err1 = VMError::StackUnderflow;
        let display = format!("{}", err1);
        assert!(display.contains("Stack underflow"));
        
        let err2 = VMError::InvalidOpCode(42);
        let display = format!("{}", err2);
        assert!(display.contains("42"));
        
        let err3 = VMError::DivisionByZero;
        let display = format!("{}", err3);
        assert!(display.contains("zero"));
    }
    
    #[test]
    fn test_error_equality() {
        assert_eq!(VMError::StackUnderflow, VMError::StackUnderflow);
        assert_ne!(VMError::StackUnderflow, VMError::DivisionByZero);
        
        assert_eq!(VMError::InvalidOpCode(5), VMError::InvalidOpCode(5));
        assert_ne!(VMError::InvalidOpCode(5), VMError::InvalidOpCode(6));
    }
    
    #[test]
    fn test_error_debug() {
        let err = VMError::OutOfBounds;
        let debug = format!("{:?}", err);
        assert_eq!(debug, "OutOfBounds");
    }
}