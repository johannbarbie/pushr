# Stack Order Analysis: Pushr vs Clojush

## The Core Issue

The fundamental difference is in how stacks are **displayed** vs how they're **stored**:

### Clojush (Reference Implementation)
- **Storage**: Uses Clojure lists where first element is top
- **Display**: When converted to vector `[2 1]`, element at index 0 is the top
- **Push order**: `(push 1) (push 2)` results in list `(2 1)` and vector `[2 1]`
- **Convention**: Top element is at the beginning

### Pushr (Current Implementation)  
- **Storage**: Uses Rust Vec where last element is top
- **Display**: Shows as `[2, 1]` but this is bottom-to-top order
- **Push order**: `push(1); push(2)` results in vec `[1, 2]` internally
- **Convention**: Top element is at the end (but displayed first)

## Detailed Operation Analysis

### 1. SWAP Operation

**Input sequence**: `1 2 SWAP`

**Clojush behavior**:
```
Push 1 → stack: (1)        → as vec: [1]
Push 2 → stack: (2 1)      → as vec: [2 1]  
SWAP   → stack: (1 2)      → as vec: [1 2]
```

**Pushr behavior**:
```
Push 1 → internal: [1]     → displayed: [1]
Push 2 → internal: [1, 2]  → displayed: [2, 1]
SWAP   → internal: [2, 1]  → displayed: [1, 2]
```

### 2. ROT Operation

**Input sequence**: `1 2 3 ROT`

**Clojush behavior**:
```
Push 1,2,3 → stack: (3 2 1)  → as vec: [3 2 1]
ROT        → stack: (1 3 2)  → as vec: [1 3 2]
```
ROT moves the 3rd element to top: 3→2→1 becomes 1→3→2

**Pushr behavior**:
```
Push 1,2,3 → internal: [1, 2, 3]  → displayed: [3, 2, 1]
ROT        → internal: [2, 3, 1]  → displayed: [1, 3, 2]
```
Using yank(2), it pulls the 3rd from top to top.

### 3. POP Operation

**Input sequence**: `1 2 3 POP`

**Clojush behavior**:
```
Push 1,2,3 → stack: (3 2 1)  → as vec: [3 2 1]
POP        → stack: (2 1)    → as vec: [2 1]
```

**Pushr behavior**:
```
Push 1,2,3 → internal: [1, 2, 3]  → displayed: [3, 2, 1]
POP        → internal: [1, 2]     → displayed: [2, 1]
```

## The Real Problem

When we run our comparison:
1. Pushr outputs its stack using `to_vec()` which gives `[1, 2, 3]` (internal order)
2. But Pushr's display shows `[3, 2, 1]` (top-to-bottom)
3. Clojush outputs `[3, 2, 1]` which represents the same logical stack

**The stacks are actually identical - we're comparing different representations!**

## Solution

The issue is in how Pushr reports its final state. The `to_vec()` method returns the internal representation (bottom-to-top), but we need to reverse it to match Clojush's top-to-bottom convention.

In `/home/johba/pushr/src/main.rs` line 46:
```rust
println!("Integer stack: {:?}", push_state.int_stack.to_vec());
```

Should be:
```rust
let mut vec = push_state.int_stack.to_vec();
vec.reverse();
println!("Integer stack: {:?}", vec);
```

This would make the output match Clojush's convention where the first element in the vector is the top of the stack.