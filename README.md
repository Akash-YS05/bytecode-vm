# Bytecode VM

A complete stack-based virtual machine implemented in Rust, featuring functions, control flow, and I/O operations.

## Architecture

### Stack-Based Design
The VM uses two separate stacks:
- **Value Stack**: Temporary computation values
- **Call Stack**: Function call frames with local variables

### Bytecode Format
Instructions are encoded as:
```
[OpCode: 1 byte][Operands: variable length]
```

Example:
```
PUSH 42    → [0][42,0,0,0,0,0,0,0]  (1 + 8 bytes)
STORE_VAR "x" → [6][1]['x']          (1 + 1 + 1 bytes)
```

## Features

### ✅ Core Operations
- Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`
- Comparisons: `GT`, `LT`, `GTE`, `LTE`, `EQ`, `NEQ`
- Variables: `STORE_VAR`, `LOAD_VAR`, `STORE_LOCAL`, `LOAD_LOCAL`

### ✅ Control Flow
- Unconditional jumps: `JUMP <address>`
- Conditional jumps: `JUMP_IF_FALSE <address>`
- Enables if/else and while loops

### ✅ Functions
- Function calls: `CALL <address>`
- Returns: `RETURN`
- Local variable scope per call frame
- Full recursion support

### ✅ I/O
- `PRINT <string>` - Print string literal
- `PRINT_VAL` - Pop and print value from stack
- `PRINT_LN` - Print newline

### ✅ Debugging
- Bytecode disassembler
- Stack traces on errors
- Debug mode with step-by-step execution
- Instruction counter

## Example Programs

### Factorial (Iterative)
```
result = 1
n = 5
while n > 0:
    result = result * n
    n = n - 1
print result  // Output: 120
```

### Fibonacci Sequence
```
a = 0
b = 1
while counter < 10:
    print a
    temp = a + b
    a = b
    b = temp
    counter = counter + 1
// Output: 0 1 1 2 3 5 8 13 21 34
```

## Technical Details

### Memory Model
- **Global Memory**: HashMap for global variables
- **Call Frames**: Stack of frames, each with local HashMap
- **Value Stack**: Vec for computation
- **Bytecode**: Vec<u8> with instruction pointer

### Performance
- O(1) instruction fetch
- O(1) stack operations
- O(1) average variable lookup
- O(n) overall execution time


## Future Enhancements

- [ ] Garbage collection
- [ ] JIT compilation
- [ ] More value types (strings, arrays)
- [ ] Standard library
- [ ] Text-based language compiler
- [ ] Optimizer (constant folding, etc.)
