use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct CallFrame {
    // return address: where to jump back to after function call
    return_address: usize,

    // local variable for this function which is seperate from global variables
    locals: HashMap<String, Value>,
}

impl CallFrame {
    pub fn new_solution(return_address: usize) -> Self {
        CallFrame {
            return_address,
            locals: HashMap::new(),
        }
    }

    pub fn store_local_solution(&mut self, name: String, value: Value) {
        self.locals.insert(name, value);
    }

    pub fn load_local_solution(&self, name: &str) -> Option<Value> {
        self.locals.get(name).copied()
    }

    pub fn return_address(&self) -> usize {
        self.return_address
    }
}