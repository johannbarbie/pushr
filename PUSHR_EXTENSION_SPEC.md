# Pushr Extension Specification for PUSH3 Circuit Implementation

## Overview

This document specifies the required extensions and improvements to the pushr library to enable full PUSH3 language support for the OpenVM-based circuit implementation. The current pushr implementation has several limitations that prevent complete PUSH3 language support.

## Current Limitations

### 1. Arithmetic Overflow Handling
- **Current**: Panics on integer overflow
- **Required**: Wrapping arithmetic behavior to match circuit implementation

### 2. Limited Opcode Support
- **Current**: Basic arithmetic and stack operations only
- **Required**: Full PUSH3 instruction set including control flow, code manipulation, and advanced stack operations

### 3. Missing Stack Types
- **Current**: INTEGER, FLOAT, BOOLEAN stacks with limited operations
- **Required**: Full support for CODE, EXEC, and NAME stacks with all specified operations

### 4. Execution Model
- **Current**: Linear execution only
- **Required**: Dynamic execution with jumps, conditionals, and loops

## Required Extensions

### 1. Arithmetic Operations Enhancement

#### 1.1 Overflow Handling
```rust
// Required changes in pushr/src/push/integer.rs
impl IntegerStack {
    pub fn add(&mut self) -> Result<(), PushError> {
        let b = self.pop()?;
        let a = self.pop()?;
        // Change from: let result = a.checked_add(b).ok_or(PushError::Overflow)?;
        let result = a.wrapping_add(b);
        self.push(result);
        Ok(())
    }
    
    // Similar changes for sub, mul, div operations
}
```

#### 1.2 Additional Arithmetic Operations
- `MOD`: Modulo operation
- `POW`: Power/exponentiation
- `ABS`: Absolute value
- `NEG`: Negation
- `MIN`: Minimum of two values
- `MAX`: Maximum of two values
- `SIGN`: Sign of value (-1, 0, 1)

### 2. Stack Manipulation Operations

#### 2.1 Basic Stack Operations
```rust
pub enum StackOperation {
    // Existing
    Push(i32),
    Pop,
    
    // Required additions
    Dup,        // Duplicate top item
    Dup2,       // Duplicate top 2 items
    Swap,       // Swap top 2 items
    Rot,        // Rotate top 3 items
    Over,       // Copy second item to top
    Drop,       // Remove top item
    Nip,        // Remove second item
    Tuck,       // Copy top item below second
    Pick(u32),  // Copy nth item to top
    Roll(u32),  // Move nth item to top
    Depth,      // Push stack depth
    Clear,      // Clear entire stack
}
```

#### 2.2 Stack Inspection
- `STACKDEPTH`: Get current stack size
- `YANKDUP`: Copy item from position
- `SHOVE`: Insert item at position

### 3. Control Flow Implementation

#### 3.1 Conditional Execution
```rust
pub struct ExecStack {
    instructions: Vec<Instruction>,
    pc: usize, // Program counter
}

pub enum ControlFlow {
    If,         // Pop condition, execute if true
    IfElse,     // Pop condition, branch
    When,       // Conditional execution
    Case,       // Multi-way branch
    Cond,       // Multiple condition-action pairs
}
```

#### 3.2 Loops
```rust
pub enum LoopOperation {
    Times,      // Execute n times
    DoLoop,     // Counted loop with index
    DoTimes,    // Simple counted loop
    While,      // Condition-based loop
    Until,      // Loop until condition true
    Forever,    // Infinite loop (with break)
    Break,      // Exit loop
    Continue,   // Continue to next iteration
}
```

### 4. CODE Stack Operations

#### 4.1 Code Manipulation
```rust
pub struct CodeStack {
    items: Vec<CodeBlock>,
}

pub enum CodeOperation {
    // List creation
    Quote,          // Start code quotation
    Unquote,        // End code quotation
    
    // Code manipulation
    Append,         // Append code blocks
    Concat,         // Concatenate code lists
    Cons,           // Prepend item to list
    Extract,        // Extract sublist
    Insert,         // Insert at position
    Remove,         // Remove from position
    
    // Code inspection
    Size,           // Get code block size
    Nth,            // Get nth instruction
    Contains,       // Check if contains instruction
    Position,       // Find instruction position
    
    // Code generation
    Rand_Code,      // Generate random code
    Mutate,         // Mutate code block
    Crossover,      // Crossover two code blocks
}
```

### 5. EXEC Stack Operations

#### 5.1 Execution Control
```rust
pub struct ExecStack {
    items: Vec<ExecutableItem>,
    call_stack: Vec<CallFrame>,
}

pub enum ExecOperation {
    // Execution
    Do,             // Execute top item
    DoStar,         // Execute repeatedly
    DoRange,        // Execute with range
    DoCount,        // Execute with counter
    
    // Stack manipulation
    Pop,            // Remove top
    Dup,            // Duplicate top
    Swap,           // Swap top two
    Rot,            // Rotate top three
    
    // Control
    K,              // Konstant combinator
    S,              // Substitution combinator
    Y,              // Y combinator
    Noop,           // No operation
}
```

### 6. NAME Stack Operations

#### 6.1 Variable Binding
```rust
pub struct NameStack {
    bindings: HashMap<String, Value>,
}

pub enum NameOperation {
    // Binding
    Quote,          // Quote name
    Bind,           // Bind value to name
    Unbind,         // Remove binding
    
    // Access
    Lookup,         // Get value by name
    Bound,          // Check if name is bound
    
    // Stack operations
    Pop,            // Remove top name
    Dup,            // Duplicate top name
    Swap,           // Swap top two names
    
    // Random names
    RandName,       // Generate random name
    NewName,        // Create unique name
}
```

### 7. Type System Extensions

#### 7.1 Float Operations (Proper Implementation)
```rust
pub struct FloatStack {
    items: Vec<f64>, // Use f64 instead of i32
}

impl FloatStack {
    pub fn push_float(&mut self, value: f64) {
        self.items.push(value);
    }
    
    // Implement all arithmetic with proper float semantics
    pub fn add(&mut self) -> Result<(), PushError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b); // Proper float addition
        Ok(())
    }
    
    // Additional float-specific operations
    pub fn sin(&mut self) -> Result<(), PushError>;
    pub fn cos(&mut self) -> Result<(), PushError>;
    pub fn tan(&mut self) -> Result<(), PushError>;
    pub fn exp(&mut self) -> Result<(), PushError>;
    pub fn ln(&mut self) -> Result<(), PushError>;
    pub fn sqrt(&mut self) -> Result<(), PushError>;
}
```

#### 7.2 Boolean Operations (Complete Set)
```rust
pub enum BooleanOperation {
    // Logical
    And,
    Or,
    Not,
    Xor,
    Nand,
    Nor,
    
    // Comparison (cross-type)
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}
```

### 8. Cross-Stack Operations

#### 8.1 Type Conversion
```rust
pub enum ConversionOperation {
    IntToFloat,
    FloatToInt,
    IntToBool,
    BoolToInt,
    FloatToBool,
    BoolToFloat,
    ToString,       // Any to string
    FromString,     // String to type
}
```

#### 8.2 Generic Stack Operations
```rust
pub trait GenericStack {
    fn flush(&mut self);              // Clear stack
    fn deep_dup(&mut self, n: usize); // Deep duplicate
    fn yank(&mut self, n: usize);     // Remove from position
    fn shove(&mut self, n: usize);    // Insert at position
}
```

### 9. Vector/List Operations

#### 9.1 Vector Support
```rust
pub struct VectorStack {
    items: Vec<Vector>,
}

pub struct Vector {
    elements: Vec<Value>,
    vector_type: VectorType,
}

pub enum VectorOperation {
    // Creation
    EmptyVector,
    BuildVector,
    
    // Access
    Nth,
    First,
    Last,
    Rest,
    ButLast,
    
    // Modification
    Append,
    Prepend,
    Concat,
    Reverse,
    Sort,
    
    // Functional
    Map,
    Filter,
    Reduce,
    ForEach,
    
    // Information
    Length,
    Contains,
    IndexOf,
}
```

### 10. Execution Trace Enhancement

#### 10.1 Enhanced Trace Information
```rust
pub struct EnhancedTrace {
    pub op_type: Push3OpType,
    pub stack_id: StackId,
    pub operands: Vec<Value>,
    pub result: Option<Value>,
    pub stack_pointer_before: usize,
    pub stack_pointer_after: usize,
    
    // New fields
    pub pc_before: usize,        // Program counter before
    pub pc_after: usize,         // Program counter after
    pub exec_stack_depth: usize, // EXEC stack depth
    pub name_bindings: HashMap<String, Value>, // Current bindings
    pub side_effects: Vec<SideEffect>, // Any side effects
}
```

#### 10.2 Control Flow Traces
```rust
pub enum ControlFlowTrace {
    Jump { from: usize, to: usize },
    ConditionalJump { condition: bool, from: usize, to: usize },
    Call { callee: String, return_addr: usize },
    Return { to: usize },
    LoopStart { counter: usize },
    LoopEnd { iterations: usize },
}
```

### 11. Error Handling Improvements

#### 11.1 Graceful Error Handling
```rust
pub enum PushError {
    StackUnderflow { stack: StackId, required: usize, available: usize },
    DivisionByZero,
    TypeError { expected: TypeId, found: TypeId },
    NameNotBound { name: String },
    InvalidInstruction { instruction: String },
    // Remove: Overflow (use wrapping arithmetic)
}

// Implement recovery strategies
pub trait ErrorRecovery {
    fn on_underflow(&mut self) -> RecoveryAction;
    fn on_type_error(&mut self) -> RecoveryAction;
}
```

### 12. Random Program Generation

#### 12.1 Genetic Programming Support
```rust
pub struct RandomCodeGenerator {
    instruction_weights: HashMap<Instruction, f64>,
    max_depth: usize,
    type_constraints: TypeConstraints,
}

impl RandomCodeGenerator {
    pub fn generate_random_program(&self, size: usize) -> Program;
    pub fn mutate_program(&self, program: &Program, rate: f64) -> Program;
    pub fn crossover_programs(&self, a: &Program, b: &Program) -> (Program, Program);
}
```

## Implementation Priority

### Phase 1: Critical Fixes (Required for basic operation)
1. Fix arithmetic overflow (use wrapping arithmetic)
2. Implement missing basic stack operations (DUP, SWAP, ROT)
3. Add proper float support

### Phase 2: Control Flow (Required for real programs)
1. Implement IF/THEN/ELSE
2. Add basic loops (TIMES, DO/LOOP)
3. Implement EXEC stack execution

### Phase 3: Advanced Features (Required for genetic programming)
1. CODE stack manipulation
2. NAME stack and bindings
3. Random code generation

### Phase 4: Optimizations
1. Efficient trace generation
2. Batch operations
3. Memory optimization

## Testing Requirements

### 1. Unit Tests
- Each operation should have comprehensive tests
- Edge cases (empty stacks, type errors)
- Overflow/underflow behavior

### 2. Integration Tests
- Complex programs using multiple features
- Genetic programming scenarios
- Large program execution (>100k operations)

### 3. Compatibility Tests
- Ensure trace format compatibility with circuit
- Verify all operations generate valid proofs

## Migration Strategy

1. **Backward Compatibility**: Maintain existing API where possible
2. **Feature Flags**: Enable new features gradually
3. **Trace Versioning**: Support both old and new trace formats
4. **Documentation**: Comprehensive docs for all new operations

## Performance Considerations

1. **Memory Efficiency**: Use efficient data structures for large stacks
2. **Execution Speed**: Optimize hot paths (arithmetic, stack operations)
3. **Trace Generation**: Minimize overhead of trace recording
4. **Parallelization**: Support parallel execution where possible

## Security Considerations

1. **Resource Limits**: Implement configurable limits on:
   - Stack depth
   - Execution steps
   - Memory usage
   - Loop iterations

2. **Deterministic Execution**: Ensure all operations are deterministic
3. **No External I/O**: Maintain pure computation model

## Conclusion

These extensions will transform pushr from a basic stack calculator into a full-featured PUSH3 interpreter suitable for genetic programming and on-chain evolution. The implementation should proceed in phases, with critical fixes first, followed by control flow, and finally advanced features for genetic programming support.