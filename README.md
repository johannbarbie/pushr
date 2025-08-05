## Pushr

![example workflow](https://github.com/johker/pushr/actions/workflows/rust.yml/badge.svg)

Pushr is a Rust based interpreter for Push programs.

## What is Push?

Push is a stack-based, Turing-complete programming language that enables autoconstructive evolution in its programs.
More information can be found [here](http://faculty.hampshire.edu/lspector/push.html).

## Supported Stack Types

This implementation supports all Push3 instructions for the types desribed in the [Push 3.0 Programming Language Description](http://faculty.hampshire.edu/lspector/push3-description.html#Type):

* BOOLEAN
* CODE
* EXECUTION
* FLOAT
* INTEGER
* NAME

Additional stack types:

* BOOLVECTOR: vector with boolean elements
* FLOATVECTOR: vector with float elements
* INTVECTOR: vector with integer elements
* INDEX: simplifies loop syntax
* GRAPH: graph object that can be used as memory

FIFO queues are used to communicate with other modules. The type is BOOLVECTOR. 
* INPUT
* OUTPUT


## Supported instructions

The default instructions for vector types are 'dup', 'equal', 'flush', 'get', 'set', 'shove', 'stackdepth', 'rand', 'swap', 'yank' and 'yankdup'. Additionally, the instruction set contains 'add', 'subtract', 'multiply' and 'divide' for float and integer vectors, as well as 'and', 'or' and 'not' for boolean vectors. To initialize vectors the instructions 'ones'  and 'zeros' can be used.

For vector instructions the following rules apply: 

* The 'rand' instruction is interpreted differently for boolean, float and integer vectors: 
   - BOOLVECTOR.RAND randomly distributes (sparsity * n) 'true' values acrross an array of length n where sparsity is the percentage of active bits.
   - INTVECTOR.RAND draws n samples from the uniform distribution U(min,max).
   - FLOATVECTOR.RAND draws n samples form the normal distribution N(mu,sig).

* Vector lengths do not have to match. Arithmetic operations are executed element-wise on the overlapping parts. An offset parameter shifts the top vector on the stack to create the desired overlap. 

* In a Push program the vectors are defined as BOOL[..], FLOAT[..] and INT[..]. For example, BOOL[1,0] defines a BOOLVECTOR with two elements. 


## Usage

### Command Line

Run Push programs directly from the command line:

```bash
cargo run -- "3 4 INTEGER.+"
cargo run -- "(1 2 3 4 5 EXEC.DO*RANGE INTEGER.*)"
```

### Library Usage

The following example shows how to interpret a Push program with Pushr:

```rust
use pushr::push::state::PushState;
use pushr::push::parser::PushParser;
use pushr::push::interpreter::PushInterpreter;
use pushr::push::instructions::{InstructionSet, InstructionCache};

// Define Push program
let input = "( CODE.QUOTE ( CODE.DUP INTEGER.DUP 1 INTEGER.- CODE.DO INTEGER.* )
               CODE.QUOTE ( INTEGER.POP 1 )
               INTEGER.DUP 2 INTEGER.< CODE.IF )";

// Create state and instruction set
let mut push_state = PushState::new();
let mut instruction_set = InstructionSet::new();
instruction_set.load();

// Create instruction cache for execution
let instruction_cache = InstructionCache::new(instruction_set.instruction_set.clone());

// Parse program onto execution stack
let parsed_program = PushParser::parse_program(&input, &instruction_cache);
push_state.exec_stack.push(parsed_program);

// Add initial values
push_state.int_stack.push(4.into());

// Run the program
PushInterpreter::run(&mut push_state, &instruction_cache);
```

For existing types the instruction set can be extended by calling the ``add`` function.


```rust
pub fn my_instruction(_push_state: &mut PushState, _instruction_set: &InstructionCache) {
    // Does nothing
}

...

let mut instruction_set = InstructionSet::new();
instruction_set.add(String::from("MyInstruction"), Instruction::new(my_instruction));
```

## Testing

Run the test suite:

```bash
cargo test
```

For verbose output:

```bash
cargo test -- --nocapture
```





