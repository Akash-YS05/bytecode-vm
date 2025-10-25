#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Integer(i64),
}

impl Value {

    //  create a new int solution
    pub fn int_solution(n: i64) -> Self {
        Value::Integer(n)
    }
    
    // convert to int solution
    pub fn as_int_solution(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
        }
    }
    
    pub fn add_solution(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Some(Value::Integer(a + b)),
        }
    }
    
    pub fn sub_solution(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Some(Value::Integer(a - b)),
        }
    }
    
    pub fn mul_solution(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Some(Value::Integer(a * b)),
        }
    }
    
    pub fn div_solution(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b == 0 {
                    None 
                } else {
                    Some(Value::Integer(a / b))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_value_creation() {
        let val = Value::int_solution(42);
        assert_eq!(val.as_int_solution(), Some(42));
    }
    
    #[test]
    fn test_value_operations() {
        let a = Value::int_solution(10);
        let b = Value::int_solution(5);
        
        assert_eq!(a.add_solution(b), Some(Value::Integer(15)));
        assert_eq!(a.sub_solution(b), Some(Value::Integer(5)));
        assert_eq!(a.mul_solution(b), Some(Value::Integer(50)));
        assert_eq!(a.div_solution(b), Some(Value::Integer(2)));
        
        let zero = Value::int_solution(0);
        assert_eq!(a.div_solution(zero), None); 
    }
    
    #[test]
    fn test_negative_numbers() {
        let a = Value::int_solution(-10);
        let b = Value::int_solution(5);
        
        assert_eq!(a.add_solution(b), Some(Value::Integer(-5)));
        assert_eq!(a.mul_solution(b), Some(Value::Integer(-50)));
    }
}