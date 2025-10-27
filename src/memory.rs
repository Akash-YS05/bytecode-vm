// Handles memory management for the VM. Adds statefullness, brings it closer to an actual programming machine

use std::collections::HashMap;
use crate::value::Value;
use crate::error::VMError;

pub struct Memory {
    // "x" -> Integer(42)
    variables: HashMap<String, Value>,
}

impl Memory {
    pub fn new_solution() -> Self {
        Memory {
            variables: HashMap::new(),
        }
    }

    pub fn store_solution(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn load_solution(&mut self, name: &str) -> Result<Value, VMError> {
        self.variables
            .get(name)
            .copied()       //copy the value (works because Value implements Copy)
            .ok_or_else(|| VMError::UndefinedVariable(name.to_string()))
    }

}