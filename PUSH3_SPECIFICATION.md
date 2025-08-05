# Push 3.0 Programming Language Specification

## Overview

Push is a stack-based programming language designed for evolutionary computation and genetic programming. Its simple syntax and multiple typed stacks enable the evolution of complex programs with arbitrary control structures, code manipulation, and modular architectures.

### Key Features
- **Multi-stack architecture**: Separate stacks for each data type (INTEGER, FLOAT, BOOLEAN, CODE, EXEC, NAME)
- **Stack-safe execution**: Instructions that lack required arguments become NOOPs
- **Code-as-data**: Programs can manipulate and generate their own code
- **Simple syntax**: Only instructions, literals, and parentheses
- **Turing-complete**: Supports recursion, iteration, and arbitrary control flow

## Language Syntax

```
program ::= instruction | literal | ( program* )
```

- **Instructions**: Named operations (e.g., `INTEGER.+`, `CODE.DO`)
- **Literals**: Constants like `42`, `3.14`, `TRUE`, `FALSE`
- **Lists**: Parenthesized sequences of programs

## Execution Model

### EXEC Stack-Based Execution

Push3 uses an explicit execution stack (EXEC) to manage program execution:

```
To execute program P:
    Push P onto the EXEC stack
    LOOP until the EXEC stack is empty:
        If the top item is an instruction → pop and execute it
        If the top item is a literal → pop and push to appropriate stack
        If the top item is a list → pop it and push all items individually in reverse order
```

This model enables:
- Reentrant interpreters (can suspend/resume execution)
- Direct manipulation of control flow via EXEC instructions
- Efficient implementation of combinators and control structures

## Data Types and Stacks

### INTEGER
- Whole numbers without decimal points
- Stack operations: arithmetic, comparisons, conversions
- Key instructions: `+`, `-`, `*`, `/`, `%`, `<`, `=`, `>`, `MAX`, `MIN`

### FLOAT
- Floating-point numbers with decimal precision
- Stack operations: arithmetic, trigonometry, conversions
- Key instructions: `+`, `-`, `*`, `/`, `%`, `SIN`, `COS`, `TAN`

### BOOLEAN
- Logical values: `TRUE` and `FALSE`
- Stack operations: logical operations, comparisons
- Key instructions: `AND`, `OR`, `NOT`, `=`

### CODE
- Push programs as data structures
- Stack operations: list manipulation, code generation, execution
- Key instructions: `DO`, `IF`, `QUOTE`, `CAR`, `CDR`, `CONS`, `RAND`

### EXEC
- Execution queue for pending operations
- Stack operations: control flow, combinators
- Key instructions: `IF`, `DO*RANGE`, `K`, `S`, `Y`

### NAME
- Symbolic identifiers for variables and defined instructions
- Stack operations: binding, lookup, manipulation
- Key instructions: `DEFINE`, `QUOTE`

## Core Instructions

### Stack Manipulation (All Types)
- `<TYPE>.DUP`: Duplicate top item
- `<TYPE>.POP`: Remove top item
- `<TYPE>.SWAP`: Swap top two items
- `<TYPE>.ROT`: Rotate top three items
- `<TYPE>.FLUSH`: Empty the stack
- `<TYPE>.STACKDEPTH`: Push stack size to INTEGER stack
- `<TYPE>.YANK`: Pull item from depth N to top
- `<TYPE>.SHOVE`: Push top item to depth N

### Arithmetic Operations
#### INTEGER Operations
- `INTEGER.+`, `INTEGER.-`, `INTEGER.*`, `INTEGER./`: Basic arithmetic
- `INTEGER.%`: Modulo operation
- `INTEGER.<`, `INTEGER.=`, `INTEGER.>`: Comparisons

#### FLOAT Operations
- `FLOAT.+`, `FLOAT.-`, `FLOAT.*`, `FLOAT./`: Basic arithmetic
- `FLOAT.%`: Modulo operation
- `FLOAT.SIN`, `FLOAT.COS`, `FLOAT.TAN`: Trigonometric functions

### Control Flow

#### Conditionals
- `CODE.IF`: Execute second or first CODE item based on BOOLEAN top
- `EXEC.IF`: Execute second or first EXEC item based on BOOLEAN top

#### Iteration
- `CODE.DO*RANGE`: Execute code for integer range (inclusive)
- `CODE.DO*COUNT`: Execute code N times with counter
- `CODE.DO*TIMES`: Execute code N times without counter
- `EXEC.DO*RANGE`, `EXEC.DO*COUNT`, `EXEC.DO*TIMES`: EXEC variants

#### Combinators
- `EXEC.K`: K combinator - removes second EXEC item
- `EXEC.S`: S combinator - complex stack rearrangement
- `EXEC.Y`: Y combinator - enables recursion

### Code Manipulation
- `CODE.QUOTE`: Push next item as code rather than executing
- `CODE.DO`: Execute top CODE item
- `CODE.APPEND`: Concatenate two code items
- `CODE.CAR`/`CODE.CDR`: List operations (first/rest)
- `CODE.CONS`: Prepend item to list
- `CODE.SIZE`: Count points in code
- `CODE.RAND`: Generate random code

### Variable Binding (NAME)
- `<TYPE>.DEFINE`: Bind NAME to value from TYPE stack
- `NAME.QUOTE`: Push name to NAME stack (even if defined)
- `CODE.DEFINITION`: Get definition of NAME

## Safety Features

### Stack Safety
- Instructions requiring N arguments with fewer than N items available → NOOP
- No crashes or exceptions from malformed programs

### Execution Limits
- **EVALPUSH-LIMIT**: Maximum execution steps per interpreter call
- **MAX-POINTS-IN-PROGRAM**: Maximum size of CODE stack items

### Safe Arithmetic
- Division by zero → NOOP
- Modulo by zero → NOOP
- All operations have defined behavior for edge cases

## Configuration Parameters

- `MIN-RANDOM-INTEGER`, `MAX-RANDOM-INTEGER`: Random integer bounds
- `MIN-RANDOM-FLOAT`, `MAX-RANDOM-FLOAT`: Random float bounds
- `MAX-POINTS-IN-RANDOM-EXPRESSIONS`: Size limit for CODE.RAND
- `MAX-POINTS-IN-PROGRAM`: Maximum program size
- `EVALPUSH-LIMIT`: Execution step limit
- `NEW-ERC-NAME-PROBABILITY`: Probability of new vs. existing random names
- `TOP-LEVEL-PUSH-CODE`: Whether to push main program to CODE stack
- `TOP-LEVEL-POP-CODE`: Whether to pop CODE stack after execution

## Random Code Generation

Standard algorithm for generating random Push programs:

```
RANDOM-CODE(MAX-POINTS):
    ACTUAL-POINTS = random(1, MAX-POINTS)
    return RANDOM-CODE-WITH-SIZE(ACTUAL-POINTS)

RANDOM-CODE-WITH-SIZE(POINTS):
    if POINTS == 1:
        return random instruction or literal
    else:
        SIZES = DECOMPOSE(POINTS - 1, random_max_parts)
        return list of RANDOM-CODE-WITH-SIZE for each size

DECOMPOSE(NUMBER, MAX-PARTS):
    recursively split NUMBER into MAX-PARTS pieces
```

## Example Programs

### Simple Arithmetic
```push
( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )
```
Result: INTEGER stack: (6), FLOAT stack: (9.3), BOOLEAN stack: (TRUE)

### Defining Instructions
```push
( DOUBLE EXEC.DEFINE ( INTEGER.DUP INTEGER.+ ) )
```
Creates DOUBLE instruction that doubles the top integer

### Factorial (Recursive)
```push
( CODE.QUOTE ( INTEGER.POP 1 )
  CODE.QUOTE ( CODE.DUP INTEGER.DUP 1 INTEGER.- CODE.DO INTEGER.* )
  INTEGER.DUP 2 INTEGER.< CODE.IF )
```

### Factorial (Iterative)
```push
( 1 INTEGER.MAX 1 EXEC.DO*RANGE INTEGER.* )
```

### Conditional Execution
```push
( INTEGER.= EXEC.IF FLOAT.* FLOAT./ )
```
Multiplies floats if integers equal, otherwise divides

### Y Combinator Loop
```push
( EXEC.Y ( <BODY/CONDITION> EXEC.IF ( ) EXEC.POP ) )
```
Implements a while loop using the Y combinator

## Implementation Requirements

### Core Components
1. **Multiple typed stacks**: INTEGER, FLOAT, BOOLEAN, CODE, EXEC, NAME
2. **Instruction set**: Type-specific and cross-type operations
3. **Execution engine**: EXEC stack-based interpreter
4. **Random code generator**: For CODE.RAND and genetic programming
5. **Safety mechanisms**: Stack safety, execution limits

### Essential Features
- Reentrant execution (can suspend/resume)
- Name binding and lookup
- Code manipulation and generation
- Full set of stack operations per type
- Type conversion instructions

## Differences from Previous Versions

### Push3 vs Push2
- Added EXEC stack for explicit execution management
- New NAME binding scheme (names with definitions act as instructions)
- Configuration code alternative to configuration files
- Added ROT and FLUSH instructions for all types

### Push2 vs Push1
- Removed instruction overloading and TYPE type
- Explicit type conversions instead of generic CONVERT
- Integrated type names into instruction names
- Changed Boolean literals from T/NIL to TRUE/FALSE

## Notes on Usage

- Arguments pop from stack tops
- Results push after arguments pop
- Stack indexing is zero-based (top = 0)
- Negative indices → 0, excessive indices → stack depth
- Instructions are case-insensitive
- Order for infix operations: second-from-top OP top (e.g., `5 3 -` = 2)