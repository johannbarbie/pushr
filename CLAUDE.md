# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Pushr is a Rust interpreter for Push3, a stack-based language for genetic programming. Currently implements basic functionality but needs major extensions for full Push3 compliance and OpenVM circuit compatibility.

**Key Resources:**
- `PUSH3_SPECIFICATION.md`: Complete language specification
- `PUSHR_EXTENSION_SPEC.md`: Detailed implementation roadmap and required fixes
- `ra_tool.md`: rust-analyzer navigation guide

## Quick Start

```bash
# Build and test
cargo build
cargo test
cargo test test_name              # Run specific test
cargo run -- "YOUR_PUSH_PROGRAM"   # Execute Push program

# Code navigation (from project root)
python3 ra_tool.py references src/push/state.rs 45 12
python3 ra_tool.py definition src/push/integer.rs 100 8
python3 ra_tool.py workspaceSymbols "PushState"
python3 ra_tool.py --format markdown hover src/main.rs 10 5
```

## Critical Implementation Tasks

### Phase 1: Fix Breaking Issues
1. **Arithmetic overflow** (src/push/integer.rs): Change from `checked_add` to `wrapping_add`
2. **Float type** (src/push/float.rs): Currently uses `i32`, needs `f64`
3. **Missing basic ops**: DUP, SWAP, ROT for all stack types

### Phase 2: Control Flow
1. Implement EXEC stack execution model (see PUSH3_SPECIFICATION.md §Execution Model)
2. Add IF/ELSE, DO*RANGE, DO*COUNT, DO*TIMES
3. Enable dynamic program execution

### Phase 3: Advanced Features
1. CODE stack manipulation (QUOTE, DO, CAR, CDR, CONS)
2. NAME bindings (DEFINE, variable lookup)
3. Random code generation for genetic programming

See `PUSHR_EXTENSION_SPEC.md` for complete implementation details.

## Architecture

**Core Files:**
- `src/push/state.rs`: PushState with all stacks (bool, int, float, code, exec, name, vectors, I/O)
- `src/push/instructions.rs`: Instruction registry and loading
- `src/push/interpreter.rs`: Execution engine (run/step)
- `src/push/parser.rs`: String → executable items

**Design Patterns:**
- Generic `PushStack<T>` for type safety
- Instructions as function pointers: `fn(&mut PushState, &InstructionCache)`
- `Item` enum for all Push values
- Tests in `#[cfg(test)]` blocks per module

## Testing Pattern

```rust
#[test]
fn test_integer_add() {
    let mut state = PushState::new();
    state.int_stack.push(3);
    state.int_stack.push(4);
    integer_add(&mut state, &InstructionCache::new(vec![]));
    assert_eq!(state.int_stack.pop(), Some(7));
}
```

## Current Limitations

1. **No wrapping arithmetic** - panics on overflow
2. **Incomplete instruction set** - missing ~60% of Push3 ops
3. **No EXEC/CODE stacks** - can't do meta-programming
4. **Linear execution only** - no conditionals/loops
5. **Wrong float type** - uses integers internally

Refer to specifications for complete Push3 requirements.