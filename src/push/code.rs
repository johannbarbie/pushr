use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::stack::PushStack;
use crate::push::state::PushState;
use crate::push::state::*;
use std::cmp;
use std::collections::HashMap;

/// For explicit code manipulation and execution. May also be used as a general list data type.
/// This type must always be present, as the top level interpreter will push any code to be
/// executed on the CODE stack prior to execution. However, one may turn off all CODE instructions
/// if code manipulation is not needed.
pub fn load_code_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("CODE.="), Instruction::new(code_eq));
    map.insert(String::from("CODE.APPEND"), Instruction::new(code_append));
    map.insert(String::from("CODE.ATOM"), Instruction::new(code_item));
    map.insert(String::from("CODE.CAR"), Instruction::new(code_first));
    map.insert(String::from("CODE.CDR"), Instruction::new(code_rest));
    map.insert(String::from("CODE.CONS"), Instruction::new(code_cons));
    map.insert(
        String::from("CODE.CONTAINER"),
        Instruction::new(code_container),
    );
    map.insert(
        String::from("CODE.CONTAINS"),
        Instruction::new(code_contains),
    );
    map.insert(String::from("CODE.DEFINE"), Instruction::new(code_define));
    map.insert(
        String::from("CODE.DEFINITION"),
        Instruction::new(code_definition),
    );
    map.insert(
        String::from("CODE.DISCREPANCY"),
        Instruction::new(code_discrepancy),
    );
    map.insert(String::from("CODE.DO"), Instruction::new(code_do));
    map.insert(String::from("CODE.DO*"), Instruction::new(code_pop_and_do));
    map.insert(String::from("CODE.DO*RANGE"), Instruction::new(code_do_range));
    map.insert(String::from("CODE.DO*COUNT"), Instruction::new(code_do_count));
    map.insert(String::from("CODE.DO*TIMES"), Instruction::new(code_do_times));
    map.insert(String::from("CODE.LOOP"), Instruction::new(code_loop));
    map.insert(String::from("CODE.DUP"), Instruction::new(code_dup));
    map.insert(String::from("CODE.OVER"), Instruction::new(code_over));
    map.insert(String::from("CODE.DROP"), Instruction::new(code_drop));
    map.insert(String::from("CODE.NIP"), Instruction::new(code_nip));
    map.insert(String::from("CODE.TUCK"), Instruction::new(code_tuck));
    map.insert(String::from("CODE.EXTRACT"), Instruction::new(code_extract));
    map.insert(String::from("CODE.FLUSH"), Instruction::new(code_flush));
    map.insert(
        String::from("CODE.FROMBOOLEAN"),
        Instruction::new(code_from_bool),
    );
    map.insert(
        String::from("CODE.FROMFLOAT"),
        Instruction::new(code_from_float),
    );
    map.insert(
        String::from("CODE.FROMINTEGER"),
        Instruction::new(code_from_int),
    );
    map.insert(
        String::from("CODE.FROMNAME"),
        Instruction::new(code_from_name),
    );
    map.insert(String::from("CODE.ID"), Instruction::new(code_id));
    map.insert(String::from("CODE.IF"), Instruction::new(code_if));
    map.insert(String::from("CODE.INSERT"), Instruction::new(code_insert));
    map.insert(String::from("CODE.LENGTH"), Instruction::new(code_length));
    map.insert(String::from("CODE.LIST"), Instruction::new(code_list));
    map.insert(String::from("CODE.MEMBER"), Instruction::new(code_member));
    map.insert(String::from("CODE.NOOP"), Instruction::new(code_noop));
    map.insert(String::from("CODE.NTH"), Instruction::new(code_nth));
    map.insert(String::from("CODE.NULL"), Instruction::new(code_null));
    map.insert(String::from("CODE.POP"), Instruction::new(code_pop));
    map.insert(
        String::from("CODE.POSITION"),
        Instruction::new(code_position),
    );
    map.insert(String::from("CODE.PRINT"), Instruction::new(code_print));
    map.insert(String::from("CODE.QUOTE"), Instruction::new(code_quote));
    map.insert(String::from("CODE.RAND"), Instruction::new(code_rand));
    map.insert(String::from("CODE.ROT"), Instruction::new(code_rot));
    map.insert(String::from("CODE.SHOVE"), Instruction::new(code_shove));
    map.insert(String::from("CODE.SIZE"), Instruction::new(code_size));
    map.insert(
        String::from("CODE.STACKDEPTH"),
        Instruction::new(code_stack_depth),
    );
    map.insert(String::from("CODE.SUBST"), Instruction::new(code_subst));
    map.insert(String::from("CODE.SWAP"), Instruction::new(code_swap));
    map.insert(String::from("CODE.YANK"), Instruction::new(code_yank));
    map.insert(
        String::from("CODE.YANKDUP"),
        Instruction::new(code_yank_dup),
    );
}

/// CODE.ID: Pushes the ID of the CODE stack to the INTEGER stack.
pub fn code_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(CODE_STACK_ID);
}

/// CODE.=: Pushes TRUE if the top two pieces of CODE are equal,
/// or FALSE otherwise.
pub fn code_eq(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.code_stack.copy_vec(2) {
        push_state
            .bool_stack
            .push(pv[0].to_string() == pv[1].to_string());
    }
}

/// CODE.APPEND: Pushes the result of appending the top two pieces of code.
/// If one of the pieces of code is a single instruction or literal (that is,
/// something not surrounded by parentheses) then it is surrounded by
/// parentheses first.
pub fn code_append(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.code_stack.pop_vec(2) {
        push_state.code_stack.push(Item::List {
            items: PushStack::from_vec(pv),
        });
    }
}

/// CODE.ATOM: Pushes TRUE onto the BOOLEAN stack if the top piece of code is a single instruction
/// or a literal, and FALSE otherwise (that is, if it is something surrounded by parentheses).
pub fn code_item(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    // Equality only checks type and ignores value
    push_state.bool_stack.push(
        push_state.code_stack.last_eq(&Item::int(0))
            || push_state.code_stack.last_eq(&Item::noop()),
    );
}

/// CODE.CAR: Pushes the first item of the list on top of the CODE stack. For example, if the top
/// piece of code is "( A B )" then this pushes "A" (after popping the argument). If the code on
/// top of the stack is not a list then this has no effect. The name derives from the similar Lisp
/// function; a more generic name would be "FIRST".
pub fn code_first(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if push_state.code_stack.last_eq(&Item::empty_list()) {
        match push_state.code_stack.pop() {
            Some(Item::List { mut items }) => {
                if let Some(item) = items.pop() {
                    push_state.code_stack.push(item);
                }
            }
            _ => (),
        }
    }
}

/// CODE.CDR: Pushes a version of the list from the top of the CODE stack without its first
/// element. For example, if the top piece of code is "( A B )" then this pushes "( B )" (after
/// popping the argument). If the code on top of the stack is not a list then this pushes the empty
/// list ("( )"). The name derives from the similar Lisp function; a more generic name would be
/// "REST".
pub fn code_rest(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    match push_state.code_stack.pop() {
        Some(Item::List { mut items }) => {
            items.pop();
            push_state.code_stack.push(Item::List { items: items });
        }
        _ => (),
    }
}

/// CODE.CONS: Pushes the result of "consing" (in the Lisp sense) the second stack item onto the
/// first stack item (which is coerced to a list if necessary). For example, if the top piece of
/// code is "( A B )" and the second piece of code is "X" then this pushes "( X A B )" (after
/// popping the argument).
pub fn code_cons(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.code_stack.pop_vec(2) {
        let mut consblock = PushStack::new();
        for i in (0..2).rev() {
            match &pv[i] {
                Item::Literal { push_type: _ } => {
                    consblock.push(pv[i].clone());
                }
                Item::List { items: a } => {
                    if let Some(vec) = a.copy_vec(a.size()) {
                        consblock.push_vec(vec)
                    }
                }
                _ => (),
            }
        }
        push_state.code_stack.push(Item::List { items: consblock });
    }
}

/// CODE.CONTAINER: Pushes the "container" of the second CODE stack item within the first CODE
/// stack item onto the CODE stack. If second item contains the first anywhere (i.e. in any nested
/// list) then the container is the smallest sub-list that contains but is not equal to the first
/// instance. For example, if the top piece of code is "( B ( C ( A ) ) ( D ( A ) ) )" and the
/// second piece of code is "( A )" then this pushes ( C ( A ) ). Pushes an empty list if there is
/// no such container.
pub fn code_container(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.copy_vec(2) {
        match Item::container(&code[1], &code[0]) {
            Ok(container) => push_state.code_stack.push(container),
            Err(_) => push_state.code_stack.push(Item::empty_list()),
        }
    }
}

/// CODE.CONTAINS: Pushes TRUE on the BOOLEAN stack if the second CODE stack item contains the
/// first CODE stack item anywhere (e.g. in a sub-list).
pub fn code_contains(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ov) = push_state.code_stack.copy_vec(2) {
        let first_el = ov[1].to_string();
        let code_str = ov[0].to_string();
        if first_el.contains(&code_str) {
            push_state.bool_stack.push(true);
        } else {
            push_state.bool_stack.push(false);
        }
    }
}

/// CODE.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the top
/// item of the CODE stack onto the EXEC stack.
pub fn code_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(instruction) = push_state.code_stack.pop() {
            push_state.name_bindings.insert(name, instruction);
        }
    }
}

/// CODE.DEFINITION: Pushes the definition associated with the top NAME on the NAME stack (if any)
/// onto the CODE stack. This extracts the definition for inspection/manipulation, rather than for
/// immediate execution (although it may then be executed with a call to CODE.DO or a similar
/// instruction).
pub fn code_definition(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(instruction) = push_state.name_bindings.get(&*name) {
            push_state.code_stack.push(instruction.clone());
        }
    }
}
/// CODE.DISCREPANCY: Pushes a measure of the discrepancy between the top two CODE stack items onto
/// the INTEGER stack. This will be zero if the top two items are equivalent, and will be higher
/// the 'more different' the items are from one another. The calculation is as follows:
/// 1. Construct a list of all of the unique items in both of the lists (where uniqueness is
///    determined by equalp). Sub-lists and items all count as items.
/// 2. Initialize the result to zero.
/// 3. For each unique item increment the result by the difference between the number of
///    occurrences of the item in the two pieces of code.
/// 4. Push the result.
pub fn code_discrepancy(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    let mut discrepancy = 0;
    if let Some(ov) = push_state.code_stack.copy_vec(2) {
        match &ov[0] {
            Item::List { items: fstlist } => {
                match &ov[1] {
                    Item::List { items: scdlist } => {
                        // Compare lists
                        if let Some(fstvec) = fstlist.copy_vec(fstlist.size()) {
                            for (i, x) in fstvec.iter().rev().enumerate() {
                                if let Some(val) = scdlist.equal_at(i, x) {
                                    if !val {
                                        discrepancy += 1;
                                    }
                                }
                            }
                        }
                        discrepancy =
                            discrepancy + (fstlist.size() as i32 - scdlist.size() as i32).abs();
                    }
                    _ => {
                        discrepancy = if ov[0].to_string() != ov[1].to_string() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
            _ => {
                discrepancy = if ov[0].to_string() != ov[1].to_string() {
                    1
                } else {
                    0
                }
            }
        }
        push_state.int_stack.push(discrepancy);
    }
}

/// CODE.DO: Recursively invokes the interpreter on the program on top of the CODE stack. After
/// evaluation the CODE stack is popped; normally this pops the program that was just executed, but
/// if the expression itself manipulates the stack then this final pop may end up popping something
/// else.
pub fn code_do(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(instruction) = push_state.code_stack.copy(0) {
        push_state.exec_stack.push(Item::InstructionMeta {
            name: "CODE.POP".to_string(),
        });
        push_state.exec_stack.push(instruction);
    }
}

/// CODE.DO*: Like CODE.DO but pops the stack before, rather than after, the recursive execution.
pub fn code_pop_and_do(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(instruction) = push_state.code_stack.copy(0) {
        push_state.exec_stack.push(instruction);
        push_state.exec_stack.push(Item::InstructionMeta {
            name: "CODE.POP".to_string(),
        });
    }
}

/// CODE.LOOP: An iteration instruction that executes the top item on the EXEC stack a number
/// of times that depends on the top two INDEX items, while also pushing the loop counter onto the
/// INDEX stack for possible access during the execution of the body of the loop.
/// First the code and the index arguments are saved locally and popped. Then the current and the
/// destination field of the index are compared. If they are equal nothing happens, i.e. the
/// index pair is just removed and the loop is terminated.
/// If the integers are not equal then the current index will be
/// pushed onto the INDEX stack but two items will be pushed onto the EXEC stack -- first a
/// recursive call to EXEC.LOOP (with the same code and destination index, but with a current
/// index that has been incremented by 1 to be closer to the destination
/// index) and then the body code.
pub fn code_loop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.code_stack.pop() {
        if let Some(index) = push_state.index_stack.copy(0) {
            if index.current < index.destination {
                let updated_loop = Item::list(vec![
                    body.clone(),
                    Item::instruction("CODE.LOOP".to_string()),
                    Item::instruction("INDEX.INCREASE".to_string()),
                ]);
                push_state.exec_stack.push(updated_loop);
                push_state.exec_stack.push(body);
            } else {
                push_state.index_stack.pop();
            }
        }
    }
}

/// CODE.DO*RANGE: Execute code for integer range (inclusive).
/// Takes end index and start index from INTEGER stack and code from CODE stack.
/// Executes code for each index from start to end, pushing current index before each execution.
pub fn code_do_range(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.code_stack.pop() {
        if let Some(end) = push_state.int_stack.pop() {
            if let Some(start) = push_state.int_stack.pop() {
                // Create an index for the range
                let index = crate::push::index::Index {
                    current: start as usize,
                    destination: end as usize,
                };
                push_state.index_stack.push(index);
                
                // If range is valid, start execution
                if start <= end {
                    let loop_body = Item::list(vec![
                        Item::instruction("INDEX.CURRENT".to_string()),
                        body.clone(),
                    ]);
                    let updated_loop = Item::list(vec![
                        loop_body,
                        Item::instruction("CODE.LOOP".to_string()),
                        Item::instruction("INDEX.INCREASE".to_string()),
                    ]);
                    push_state.exec_stack.push(updated_loop);
                } else {
                    push_state.index_stack.pop();
                }
            }
        }
    }
}

/// CODE.DO*COUNT: Execute code N times with counter.
/// Takes count from INTEGER stack and code from CODE stack.
/// Executes code count times, pushing counter (0 to count-1) before each execution.
pub fn code_do_count(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.code_stack.pop() {
        if let Some(count) = push_state.int_stack.pop() {
            if count > 0 {
                // Create an index from 0 to count-1
                let index = crate::push::index::Index {
                    current: 0,
                    destination: (count - 1) as usize,
                };
                push_state.index_stack.push(index);
                
                let loop_body = Item::list(vec![
                    Item::instruction("INDEX.CURRENT".to_string()),
                    body.clone(),
                ]);
                let updated_loop = Item::list(vec![
                    loop_body,
                    Item::instruction("CODE.LOOP".to_string()),
                    Item::instruction("INDEX.INCREASE".to_string()),
                ]);
                push_state.exec_stack.push(updated_loop);
            }
        }
    }
}

/// CODE.DO*TIMES: Execute code N times without counter.
/// Takes count from INTEGER stack and code from CODE stack.
/// Executes code count times without pushing any counter.
pub fn code_do_times(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.code_stack.pop() {
        if let Some(count) = push_state.int_stack.pop() {
            if count > 0 {
                // Create an index from 0 to count-1 (for internal counting only)
                let index = crate::push::index::Index {
                    current: 0,
                    destination: (count - 1) as usize,
                };
                push_state.index_stack.push(index);
                
                let updated_loop = Item::list(vec![
                    body.clone(),
                    Item::instruction("CODE.LOOP".to_string()),
                    Item::instruction("INDEX.INCREASE".to_string()),
                ]);
                push_state.exec_stack.push(updated_loop);
            }
        }
    }
}

/// CODE.DUP: Duplicates the top item on the CODE stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn code_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(instruction) = push_state.code_stack.copy(0) {
        push_state.code_stack.push(instruction);
    }
}

/// CODE.OVER: Copies the second item and pushes it on top of the stack.
pub fn code_over(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(items) = push_state.code_stack.copy_vec(2) {
        push_state.code_stack.push(items[0].clone());
    }
}

/// CODE.DROP: Removes the top item from the stack.
pub fn code_drop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.code_stack.pop();
}

/// CODE.NIP: Removes the second item from the stack.
pub fn code_nip(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(top) = push_state.code_stack.pop() {
        push_state.code_stack.pop();
        push_state.code_stack.push(top);
    }
}

/// CODE.TUCK: Copies the top item and inserts it before the second item.
pub fn code_tuck(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut items) = push_state.code_stack.pop_vec(2) {
        let top = items.pop().unwrap();
        let second = items.pop().unwrap();
        push_state.code_stack.push(top.clone());
        push_state.code_stack.push(second);
        push_state.code_stack.push(top);
    }
}

/// CODE.EXTRACT: Pushes the sub-expression of the top item of the CODE stack that is indexed by
/// the top item of the INTEGER stack. The indexing here counts "points," where each parenthesized
/// expression and each literal/instruction is considered a point, and it proceeds in depth first
/// order. The entire piece of code is at index 0; if it is a list then the first item in the list
/// is at index 1, etc. The integer used as the index is taken modulo the number of points in the
/// overall expression (and its absolute value is taken in case it is negative) to ensure that it
/// is within the meaningful range.
pub fn code_extract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(sub_idx) = push_state.int_stack.pop() {
        if let Some(code) = push_state.code_stack.get(0) {
            let total_size = Item::size(code);
            let norm_idx = sub_idx.rem_euclid(total_size as i32);
            match Item::traverse(&code, norm_idx as usize) {
                Ok(el) => push_state.code_stack.push(el),
                Err(_) => (),
            };
        }
    }
}

/// CODE.FLUSH: Empties the CODE stack.
pub fn code_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.code_stack.flush();
}

/// CODE.FROMBOOLEAN: Pops the BOOLEAN stack and pushes the popped item (TRUE or FALSE) onto the
/// CODE stack.
pub fn code_from_bool(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bval) = push_state.bool_stack.pop() {
        push_state.code_stack.push(Item::bool(bval));
    }
}
/// CODE.FROMFLOAT: Pops the FLOAT stack and pushes the popped item onto the CODE stack.
pub fn code_from_float(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.code_stack.push(Item::float(fval));
    }
}

/// CODE.FROMINTEGER: Pops the INTEGER stack and pushes the popped integer onto the CODE stack.
pub fn code_from_int(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.code_stack.push(Item::int(ival));
    }
}

/// CODE.FROMNAME: Pops the NAME stack and pushes the popped item onto the CODE stack.
pub fn code_from_name(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(nval) = push_state.name_stack.pop() {
        push_state.code_stack.push(Item::id(nval.to_string()));
    }
}

/// CODE.IF: If the top item of the BOOLEAN stack is TRUE this recursively executes the second item
/// of the CODE stack; otherwise it recursively executes the first item of the CODE stack. Either
/// way both elements of the CODE stack (and the BOOLEAN value upon which the decision was made)
/// are popped.
pub fn code_if(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.pop_vec(2) {
        if let Some(exec_second) = push_state.bool_stack.pop() {
            if exec_second {
                // Push second element for execution
                push_state.exec_stack.push(code[0].clone());
            } else {
                // Push first element for execution
                push_state.exec_stack.push(code[1].clone());
            }
        }
    }
}

/// CODE.INSERT: Pushes the result of inserting the second item of the CODE stack into the first
/// item, at the position indexed by the top item of the INTEGER stack (and replacing whatever was
/// there formerly). The indexing is computed as in CODE.EXTRACT.
pub fn code_insert(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(sub_idx) = push_state.int_stack.pop() {
        if let Some(code_to_be_inserted) = push_state.code_stack.copy(1) {
            let _ = Item::insert(
                push_state.code_stack.get_mut(0).unwrap(),
                &code_to_be_inserted,
                sub_idx as usize,
            );
        }
    }
}

/// CODE.LENGTH: Pushes the length of the top item on the CODE stack onto the INTEGER stack. If the
/// top item is not a list then this pushes a 1. If the top item is a list then this pushes the
/// number of items in the top level of the list; that is, nested lists contribute only 1 to this
/// count, no matter what they contain.
pub fn code_length(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(top_item) = push_state.code_stack.get(0) {
        match top_item {
            Item::List { items } => push_state.int_stack.push(items.size() as i32),
            _ => push_state.int_stack.push(1),
        }
    }
}

/// CODE.LIST: Pushes a list of the top two items of the CODE stack onto the CODE stack.
pub fn code_list(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(top_items) = push_state.code_stack.copy_vec(2) {
        push_state
            .code_stack
            .push(Item::list(vec![top_items[0].clone(), top_items[1].clone()]));
    }
}

/// CODE.CONTAINS: Pushes TRUE on the BOOLEAN stack if the second CODE stack item contains the
/// first CODE stack item anywhere (e.g. in a sub-list).
pub fn code_member(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ov) = push_state.code_stack.copy_vec(2) {
        let top_el = ov[1].to_string();
        let sec_el = ov[0].to_string();
        if sec_el.contains(&top_el) {
            push_state.bool_stack.push(true);
        } else {
            push_state.bool_stack.push(false);
        }
    }
}

/// CODE.NOOP: Does nothing.
pub fn code_noop(_push_state: &mut PushState, _instruction_cache: &InstructionCache) {}

/// CODE.NTH: Pushes the nth element of the expression on top of the CODE stack (which is coerced
/// to a list first if necessary). If the expression is an empty list then the result is an empty
/// list. N is taken from the INTEGER stack and is taken modulo the length of the expression into
/// which it is indexing.
pub fn code_nth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(sub_idx) = push_state.int_stack.pop() {
        if let Some(code) = push_state.code_stack.get(0) {
            let total_size = Item::shallow_size(code);
            let idx = sub_idx.rem_euclid(total_size as i32);
            let mut item_to_push = Item::empty_list();
            if idx == 0 {
                item_to_push = code.clone();
            }
            match code {
                Item::List { items } => {
                    if let Some(nth_item) = items.get(idx as usize - 1) {
                        item_to_push = nth_item.clone();
                    }
                }
                _ => (),
            }
            push_state.code_stack.push(item_to_push);
        }
    }
}

/// CODE.NULL: Pushes TRUE onto the BOOLEAN stack if the top item of the CODE stack is an empty
/// list, or FALSE otherwise.
pub fn code_null(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.get(0) {
        let mut is_null = false;
        match code {
            Item::List { items } => {
                if items.size() == 0 {
                    is_null = true;
                }
            }
            _ => (),
        }
        push_state.bool_stack.push(is_null);
    }
}

/// CODE.POP: Pops the CODE stack.
pub fn code_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.code_stack.pop();
}

/// CODE.POSITION: Pushes onto the INTEGER stack the position of the second item on the CODE stack
/// within the first item (which is coerced to a list if necessary). Pushes -1 if no match is found.
pub fn code_position(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.copy_vec(2) {
        match Item::contains(&code[1], &code[0], 0) {
            Ok(pos) => push_state.int_stack.push(pos as i32),
            Err(()) => push_state.int_stack.push(-1),
        }
    }
}

/// CODE.PRINT: Pushes the string representation of the code stack to the name stack. 
pub fn code_print(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if push_state.code_stack.size() > 0 {
        let code_str = push_state.code_stack.to_string();
        push_state.name_stack.push(code_str);

    }
}

/// CODE.QUOTE: Specifies that the next expression submitted for execution will instead be pushed
/// literally onto the CODE stack. This can be implemented by moving the top item on the EXEC stack
/// onto the CODE stack.
pub fn code_quote(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(exec_code) = push_state.exec_stack.pop() {
        push_state.code_stack.push(exec_code);
    }
}

/// CODE.RAND: Pushes a newly-generated random program onto the CODE stack. The limit for the size
/// of the expression is taken from the INTEGER stack; to ensure that it is in the appropriate
/// range this is taken modulo the value of the MAX-POINTS-IN-RANDOM-EXPRESSIONS parameter and the
/// absolute value of the result is used.
pub fn code_rand(push_state: &mut PushState, instruction_cache: &InstructionCache) {
    if let Some(size_limit) = push_state.int_stack.pop() {
        let limit = cmp::min(
            i32::abs(size_limit),
            i32::abs(push_state.configuration.max_points_in_random_expressions),
        );
        if let Some(rand_item) =
            CodeGenerator::random_code(&push_state, &instruction_cache, limit as usize)
        {
            push_state.code_stack.push(rand_item);
        }
    }
}

/// CODE.ROT: Rotates the top three items on the CODE stack, pulling the third item out and pushing
/// it on top. This is equivalent to "2 CODE.YANK".
pub fn code_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.code_stack.yank(2);
}

/// CODE.SHOVE: Inserts the top piece of CODE "deep" in the stack, at the position indexed by the
/// top INTEGER.
pub fn code_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.code_stack.size() as i32) - 1, shove_index),
            0,
        ) as usize;
        push_state.code_stack.shove(corr_index as usize);
    }
}

/// CODE.SIZE: Pushes the number of "points" in the top piece of CODE onto the INTEGER stack. Each
/// instruction, literal, and pair of parentheses counts as a point.
pub fn code_size(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.get(0) {
        push_state.int_stack.push(Item::size(&code) as i32);
    }
}

/// CODE.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn code_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.code_stack.size() as i32);
}

/// CODE.SUBST: Pushes the result of substituting the third item on the code stack for the second
/// item in the first item. As of this writing this is implemented only in the Lisp implementation,
/// within which it relies on the Lisp "subst" function. As such, there are several problematic
/// possibilities; for example "dotted-lists" can result in certain cases with empty-list
/// arguments. If any of these problematic possibilities occurs the stack is left unchanged.
pub fn code_subst(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.code_stack.pop_vec(3) {
        // code[2]: first item => item to be modified (target)
        // code[1]: second item => substitute
        // code[0]: third item => replace pattern
        let mut target = code[2].clone();
        if Item::substitute(&mut target, &code[0], &code[1]) {
            // Target and pattern are the same => push substitute
            push_state.code_stack.push(code[1].clone());
        } else {
            // Push target with substitute
            push_state.code_stack.push(target);
        }
    }
}

/// CODE.SWAP: Swaps the top two pieces of CODE.
pub fn code_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.code_stack.shove(1);
}

/// CODE.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the stack.
/// The index is taken from the INTEGER stack.
pub fn code_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.code_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        push_state.code_stack.yank(corr_index as usize);
    }
}

/// CODE.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the stack,
/// without removing the deep item. The index is taken from the INTEGER stack.
pub fn code_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.code_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.code_stack.copy(corr_index as usize) {
            push_state.code_stack.push(deep_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::index::Index;
    use crate::push::parser::PushParser;
    use crate::push::instructions::InstructionSet;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn code_eq_pushes_true_when_elements_equal() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(1));
        code_eq(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.size(), 2);
        if let Some(val) = test_state.bool_stack.pop() {
            assert_eq!(val, true, "Must be true in case of equality");
        } else {
            assert!(false, "Expected bool value");
        }
    }

    #[test]
    fn code_eq_pushes_false_when_elements_unequal() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        code_eq(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.size(), 2);
        if let Some(val) = test_state.bool_stack.pop() {
            assert_eq!(val, false, "Must be false in case of inequality");
        } else {
            assert!(false, "Expected bool value");
        }
    }

    #[test]
    fn code_append_pushes_block_when_finding_literals() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        code_append(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.size(), 1, "Excpected single element");
        assert!(
            test_state.code_stack.last_eq(&Item::empty_list()),
            "Expected Code Block"
        );
    }

    #[test]
    fn code_item_pushes_true_when_no_list_found() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(0));
        code_item(&mut test_state, &icache());
        assert!(
            test_state.bool_stack.last_eq(&true),
            "Should push true for Literal"
        );
        test_state = PushState::new();
        test_state.code_stack.push(Item::noop());
        code_item(&mut test_state, &icache());
        assert!(
            test_state.bool_stack.last_eq(&true),
            "Should push true for Instruction"
        );
        test_state = PushState::new();
        test_state.code_stack.push(Item::empty_list());
        code_item(&mut test_state, &icache());
        assert!(
            test_state.bool_stack.last_eq(&false),
            "Should push false for Code Block"
        );
    }

    #[test]
    fn code_first_pushes_first_element_when_cb_is_found() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2), Item::int(3)]));
        code_first(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "3");
    }

    #[test]
    fn code_rest_pushes_all_except_first_element() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2), Item::int(3)]));
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 3 2 1 )"
        );
        code_rest(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 2 1 )"
        );
    }

    #[test]
    fn code_cons_appends_in_reverse_order() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        assert_eq!(
            test_state.code_stack.to_string(),
            "2 1"
        );
        code_cons(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 1 2 )"
        );
    }

    #[test]
    fn code_container_finds_subelement() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        test_state.code_stack.push(Item::list(vec![
            Item::list(vec![
                Item::int(3),
                Item::list(vec![Item::int(1), Item::int(2)]),
                Item::list(vec![Item::int(3), Item::int(3)]),
                Item::int(3),
            ]),
            Item::int(4),
            Item::int(5),
        ]));
        code_container(&mut test_state, &icache());
        // The top element is expected to be the smallest container of (1 2)' => (3 (1 2)' (3 3)' 3)'
        assert!(test_state
            .code_stack
            .to_string()
            .starts_with("( 3 ( 3 3 ) ( 2 1 ) 3"));
    }

    #[test]
    fn code_contains_pushes_true_if_second_contains_first() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        test_state.code_stack.push(Item::list(vec![
            Item::list(vec![
                Item::int(3),
                Item::list(vec![Item::int(1), Item::int(2)]),
                Item::list(vec![Item::int(3), Item::int(3)]),
                Item::int(3),
            ]),
            Item::int(4),
            Item::int(5),
        ]));
        code_contains(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "TRUE");
    }

    #[test]
    fn code_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(2));
        test_state.name_stack.push(String::from("TEST"));
        code_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::int(2).to_string()
        );
    }

    #[test]
    fn code_definition_pushes_to_code_stack() {
        let mut test_state = PushState::new();
        test_state
            .name_bindings
            .insert(String::from("TEST"), Item::int(2));
        test_state.name_stack.push(String::from("TEST"));
        code_definition(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.pop().unwrap().to_string(),
            Item::int(2).to_string()
        );
    }

    #[test]
    fn code_discrepancy_calculates_zero_discrepancy_correctly() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        code_discrepancy(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "0");
    }

    #[test]
    fn code_discrepancy_calculates_discrepancy_correctly() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(0), Item::int(2)]));
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        code_discrepancy(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1");
    }

    #[test]
    fn code_do_adds_instruction_to_excecution_stack() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(9));
        code_do(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "9 CODE.POP"
        );
    }

    #[test]
    fn code_pop_and_do_adds_instruction_to_excecution_stack() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(9));
        code_pop_and_do(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "CODE.POP 9"
        );
    }

    #[test]
    fn code_loop_pushes_body_and_updated_loop() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::noop());
        test_state.index_stack.push(Index::new(3));
        code_loop(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "NOOP ( INDEX.INCREASE CODE.LOOP NOOP )");
    }

    #[test]
    fn code_loop_removes_index_when_terminated() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::noop());
        let mut test_index = Index::new(3);
        test_index.current = 3;
        test_state.index_stack.push(test_index);
        code_loop(&mut test_state, &icache());
        assert_eq!(test_state.index_stack.to_string(), "");
        assert_eq!(test_state.exec_stack.to_string(), "");
    }
    
    #[test]
    fn code_do_range_creates_loop_with_index() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("INTEGER.+".to_string()));
        test_state.int_stack.push(1); // start
        test_state.int_stack.push(3); // end
        code_do_range(&mut test_state, &icache());
        
        // Should create index 1/3 and push loop structure
        assert_eq!(test_state.index_stack.to_string(), "1/3");
        assert_eq!(test_state.exec_stack.to_string(), "( INDEX.INCREASE CODE.LOOP ( INTEGER.+ INDEX.CURRENT ) )");
    }
    
    #[test]
    fn code_do_range_invalid_range() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("INTEGER.+".to_string()));
        test_state.int_stack.push(5); // start > end
        test_state.int_stack.push(3); // end
        code_do_range(&mut test_state, &icache());
        
        // Should not create any loop
        assert_eq!(test_state.index_stack.to_string(), "");
        assert_eq!(test_state.exec_stack.to_string(), "");
    }
    
    #[test]
    fn code_do_count_creates_loop_with_counter() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("INTEGER.DUP".to_string()));
        test_state.int_stack.push(3); // count
        code_do_count(&mut test_state, &icache());
        
        // Should create index 0/2 (0 to count-1)
        assert_eq!(test_state.index_stack.to_string(), "0/2");
        assert_eq!(test_state.exec_stack.to_string(), "( INDEX.INCREASE CODE.LOOP ( INTEGER.DUP INDEX.CURRENT ) )");
    }
    
    #[test]
    fn code_do_count_zero_count() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("INTEGER.DUP".to_string()));
        test_state.int_stack.push(0); // count = 0
        code_do_count(&mut test_state, &icache());
        
        // Should not create any loop
        assert_eq!(test_state.index_stack.to_string(), "");
        assert_eq!(test_state.exec_stack.to_string(), "");
    }
    
    #[test]
    fn code_do_times_creates_loop_without_counter() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("BOOLEAN.NOT".to_string()));
        test_state.int_stack.push(2); // times
        code_do_times(&mut test_state, &icache());
        
        // Should create index 0/1 but not push counter in loop
        assert_eq!(test_state.index_stack.to_string(), "0/1");
        assert_eq!(test_state.exec_stack.to_string(), "( INDEX.INCREASE CODE.LOOP BOOLEAN.NOT )");
    }
    
    #[test]
    fn code_do_times_zero_times() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::instruction("BOOLEAN.NOT".to_string()));
        test_state.int_stack.push(0); // times = 0
        code_do_times(&mut test_state, &icache());
        
        // Should not create any loop
        assert_eq!(test_state.index_stack.to_string(), "");
        assert_eq!(test_state.exec_stack.to_string(), "");
    }

    #[test]
    fn code_dup_duplicates_top_element() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::noop());
        code_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "NOOP NOOP"
        );
    }

    #[test]
    fn code_flush_empties_stack() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(0), Item::int(2)]));
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        code_flush(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "");
    }

    #[test]
    fn code_from_bool_pushes_literal() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        code_from_bool(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "TRUE");
    }

    #[test]
    fn code_if_pushes_second_item_when_true() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        code_if(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "2");
        assert_eq!(test_state.code_stack.to_string(), "");
        assert_eq!(test_state.bool_stack.to_string(), "");
    }

    #[test]
    fn code_if_pushes_first_item_when_false() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(false);
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        code_if(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "1");
        assert_eq!(test_state.code_stack.to_string(), "");
        assert_eq!(test_state.bool_stack.to_string(), "");
    }

    #[test]
    fn code_extract_finds_correct_subelement() {
        let mut test_state = PushState::new();
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        // Total Size = 6 => 10 % 6 = 4
        // Expected 4th element - Literal(3) - to be extracted
        test_state.int_stack.push(10);
        test_state.code_stack.push(test_item);
        code_extract(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
        assert_eq!(
            test_state.code_stack.to_string(),
            "3 ( 1 2 ( 3 ) 4 )"
        );
    }

    #[test]
    fn code_insert_replaces_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        let test_container = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let test_item = Item::int(5);
        test_state.code_stack.push(test_item);
        test_state.code_stack.push(test_container);
        code_insert(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 1 2 ( 5 ) 4 ) 5"
        );
    }

    #[test]
    fn code_insert_does_nothing_when_index_too_big() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        let test_container = Item::list(vec![Item::int(2), Item::int(1)]);
        let test_item = Item::int(5);
        test_state.code_stack.push(test_item);
        test_state.code_stack.push(test_container);
        code_insert(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 1 2 ) 5"
        );
    }

    #[test]
    fn code_length_pushes_top_list_size() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::list(vec![
            Item::int(2),
            Item::int(1),
            Item::list(vec![Item::int(0), Item::float(2.3)]),
        ]));
        code_length(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3");
    }

    #[test]
    fn code_list_pushes_lists_including_top_items() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(0), Item::float(2.3)]));
        test_state.code_stack.push(Item::int(2));
        code_list(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "( 2 ( 2.300 0 ) ) 2 ( 2.300 0 )");
    }

    #[test]
    fn code_nth_ignores_nested_lists() {
        let mut test_state = PushState::new();
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        // Shallow Size = 5 => 9 % 5 = 4
        // Expected 4th element - Literal(4) - to be extracted
        test_state.int_stack.push(9);
        test_state.code_stack.push(test_item);
        code_nth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
        assert_eq!(
        test_state.code_stack.to_string(),
        "4 ( 1 2 ( 3 ) 4 )"
    );
    }

    #[test]
    fn code_null_pushes_true_for_empty_list() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::empty_list());
        code_null(&mut test_state, &icache());
        assert_eq!(*test_state.bool_stack.get(0).unwrap(), true);
    }

    #[test]
    fn code_pop_removes_top_element() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(3));
        code_pop(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "2 1"
        );
    }

    #[test]
    fn code_position_pushes_value_when_contained() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]));
        code_position(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.get(0).unwrap(), &4);
    }

    #[test]
    fn code_print_creates_parseable_output() {
        let mut test_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]));
        code_print(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.size(), 1);
        let printed_code = test_state.name_stack.copy(0).unwrap();
        PushParser::parse_program(&mut test_state, &instruction_set, &printed_code);
        assert_eq!(
            test_state.exec_stack.to_string(), test_state.code_stack.to_string());
    }

    #[test]
    fn code_quote_moves_item_from_exec_to_code_stack() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(2));
        code_quote(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "2")
    }

    #[test]
    fn code_rand_pushes_random_code() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(100);
        code_rand(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.size(), 1);
    }

    #[test]
    fn code_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1 2 3"
        );
        code_rot(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "3 1 2"
        );
    }

    #[test]
    fn code_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(4));
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1 2 3 4"
        );
        test_state.int_stack.push(2);
        code_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "2 3 1 4"
        );
    }

    #[test]
    fn code_size_calculates_top_element() {
        let mut test_state = PushState::new();
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        test_state.code_stack.push(test_item);
        code_size(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "6");
    }

    #[test]
    fn code_substitute_code_elements() {
        let mut test_state = PushState::new();
        let target_item = Item::list(vec![
            Item::list(vec![]),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let substitute = Item::int(4);
        let pattern = Item::list(vec![]);
        test_state.code_stack.push(pattern);
        test_state.code_stack.push(substitute);
        test_state.code_stack.push(target_item);
        code_subst(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "( 1 2 ( 3 ) 4 )"
        );
    }

    #[test]
    fn code_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(0));
        test_state.code_stack.push(Item::int(1));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1 0"
        );
        code_swap(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "0 1"
        );
    }

    #[test]
    fn code_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(5));
        test_state.code_stack.push(Item::int(4));
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1 2 3 4 5"
        );
        test_state.int_stack.push(3);
        code_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "4 1 2 3 5"
        );
    }

    #[test]
    fn code_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(5));
        test_state.code_stack.push(Item::int(4));
        test_state.code_stack.push(Item::int(3));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(1));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1 2 3 4 5"
        );
        test_state.int_stack.push(3);
        code_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "4 1 2 3 4 5"
        );
    }
    
    #[test]
    fn code_from_float_pushes_float_literal() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(3.14);
        code_from_float(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "3.140");
        assert_eq!(test_state.float_stack.size(), 0);
    }
    
    #[test]
    fn code_from_int_pushes_int_literal() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(42);
        code_from_int(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "42");
        assert_eq!(test_state.int_stack.size(), 0);
    }
    
    #[test]
    fn code_from_name_pushes_name_as_code() {
        let mut test_state = PushState::new();
        test_state.name_stack.push("TESTNAME".to_string());
        code_from_name(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "TESTNAME");
        assert_eq!(test_state.name_stack.size(), 0);
    }
    
    #[test]
    fn code_member_checks_if_first_in_second() {
        let mut test_state = PushState::new();
        // Check if (1 2 3) contains 2
        // Per the implementation: checks if second (ov[0]) contains first (ov[1])
        test_state.code_stack.push(Item::list(vec![
            Item::int(1),
            Item::int(2),
            Item::int(3),
        ]));
        test_state.code_stack.push(Item::int(2));
        code_member(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
        
        // Check if (1 2 3) contains 4
        test_state.code_stack.push(Item::list(vec![
            Item::int(1),
            Item::int(2),
            Item::int(3),
        ]));
        test_state.code_stack.push(Item::int(4));
        code_member(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
    }
    
    #[test]
    fn code_noop_does_nothing() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(42);
        test_state.code_stack.push(Item::int(1));
        test_state.bool_stack.push(true);
        
        let int_before = test_state.int_stack.to_string();
        let code_before = test_state.code_stack.to_string();
        let bool_before = test_state.bool_stack.to_string();
        
        code_noop(&mut test_state, &icache());
        
        assert_eq!(test_state.int_stack.to_string(), int_before);
        assert_eq!(test_state.code_stack.to_string(), code_before);
        assert_eq!(test_state.bool_stack.to_string(), bool_before);
    }
    
    #[test]
    fn code_stack_depth_pushes_size() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::list(vec![Item::int(3), Item::int(4)]));
        
        code_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 3);
        assert_eq!(test_state.code_stack.size(), 3); // Stack unchanged
    }
    
    #[test]
    fn code_over_copies_second_item() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(3));
        assert_eq!(test_state.code_stack.to_string(), "3 2 1");
        
        code_over(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "2 3 2 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(5));
        code_over(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "5");
    }
    
    #[test]
    fn code_drop_removes_top() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(3));
        
        code_drop(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "2 1");
        
        // Test with empty stack
        let mut test_state = PushState::new();
        code_drop(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "");
    }
    
    #[test]
    fn code_nip_removes_second() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(3));
        
        code_nip(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "3 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(5));
        code_nip(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "5");
    }
    
    #[test]
    fn code_tuck_inserts_copy() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::int(2));
        test_state.code_stack.push(Item::int(3));
        
        code_tuck(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "3 2 3 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(5));
        code_tuck(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "5");
    }
}
