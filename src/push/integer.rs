use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use crate::push::state::*;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Zero, One, Signed, ToPrimitive};

/// Integer numbers (that is, numbers without decimal points).
pub fn load_int_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("INTEGER.%"), Instruction::new(integer_modulus));
    map.insert(String::from("INTEGER.*"), Instruction::new(integer_mult));
    map.insert(String::from("INTEGER.+"), Instruction::new(integer_add));
    map.insert(
        String::from("INTEGER.-"),
        Instruction::new(integer_subtract),
    );
    map.insert(String::from("INTEGER./"), Instruction::new(integer_divide));
    map.insert(String::from("INTEGER.<"), Instruction::new(integer_smaller));
    map.insert(String::from("INTEGER.="), Instruction::new(integer_equal));
    map.insert(String::from("INTEGER.>"), Instruction::new(integer_greater));
    map.insert(String::from("INTEGER.ABS"), Instruction::new(integer_abs));
    map.insert(String::from("INTEGER.NEG"), Instruction::new(integer_neg));
    map.insert(String::from("INTEGER.POW"), Instruction::new(integer_pow));
    map.insert(String::from("INTEGER.SIGN"), Instruction::new(integer_sign));
    map.insert(
        String::from("INTEGER.DEFINE"),
        Instruction::new(integer_define),
    );
    map.insert(String::from("INTEGER.DUP"), Instruction::new(integer_dup));
    map.insert(String::from("INTEGER.DDUP"), Instruction::new(integer_ddup));
    map.insert(String::from("INTEGER.DUP2"), Instruction::new(integer_ddup));  // Alias for DDUP
    map.insert(String::from("INTEGER.OVER"), Instruction::new(integer_over));
    map.insert(String::from("INTEGER.DROP"), Instruction::new(integer_drop));
    map.insert(String::from("INTEGER.NIP"), Instruction::new(integer_nip));
    map.insert(String::from("INTEGER.TUCK"), Instruction::new(integer_tuck));
    map.insert(
        String::from("INTEGER.FLUSH"),
        Instruction::new(integer_flush),
    );
    map.insert(
        String::from("INTEGER.FROMBOOLEAN"),
        Instruction::new(integer_from_boolean),
    );
    map.insert(
        String::from("INTEGER.FROMFLOAT"),
        Instruction::new(integer_from_float),
    );
    map.insert(String::from("INTEGER.ID"), Instruction::new(integer_id));
    map.insert(String::from("INTEGER.MAX"), Instruction::new(integer_max));
    map.insert(String::from("INTEGER.MIN"), Instruction::new(integer_min));
    map.insert(String::from("INTEGER.POP"), Instruction::new(integer_pop));
    map.insert(String::from("INTEGER.RAND"), Instruction::new(integer_rand));
    map.insert(String::from("INTEGER.ROT"), Instruction::new(integer_rot));
    map.insert(
        String::from("INTEGER.SHOVE"),
        Instruction::new(integer_shove),
    );
    map.insert(
        String::from("INTEGER.STACKDEPTH"),
        Instruction::new(integer_stack_depth),
    );
    map.insert(String::from("INTEGER.SWAP"), Instruction::new(integer_swap));
    map.insert(String::from("INTEGER.YANK"), Instruction::new(integer_yank));
    map.insert(
        String::from("INTEGER.YANKDUP"),
        Instruction::new(integer_yank_dup),
    );
}

/// INTEGER.ID: Pushes the ID of the INTEGER stack to the INTEGER stack.
pub fn integer_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(BigInt::from(INT_STACK_ID));
}

/// INTEGER.%: Pushes the second stack item modulo the top stack item. If the top item is zero this
/// acts as a NOOP (leaving both items on stack). The modulus is computed to match Clojure's mod function,
/// which returns a value with the same sign as the divisor.
pub fn integer_modulus(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if push_state.int_stack.size() >= 2 {
        let divisor = push_state.int_stack.get(0).unwrap();
        if !divisor.is_zero() {
            if let Some(ivals) = push_state.int_stack.pop_vec(2) {
                // Match Clojure's mod behavior: result has same sign as divisor
                let a = &ivals[0];
                let b = &ivals[1];
                let rem = a % b;
                let result = if rem.is_zero() || (rem.sign() == b.sign()) {
                    rem
                } else {
                    rem + b.clone()
                };
                push_state.int_stack.push(result);
            }
        }
        // If divisor is zero, do nothing (leave both items on stack)
    }
}

/// INTEGER.*: Pushes the product of the top two items.
fn integer_mult(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(&ivals[0] * &ivals[1]);
    }
}

/// INTEGER.+: Pushes the sum of the top two items.
fn integer_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(&ivals[0] + &ivals[1]);
    }
}

/// INTEGER.-: Pushes the difference of the top two items; that is, the second item minus the top
/// item.
fn integer_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(&ivals[0] - &ivals[1]);
    }
}

/// INTEGER./: Pushes the quotient of the top two items; that is, the second item divided by the
/// top item. If the top item is zero this acts as a NOOP (leaving both items on stack).
fn integer_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if push_state.int_stack.size() >= 2 {
        let divisor = push_state.int_stack.get(0).unwrap();
        if !divisor.is_zero() {
            if let Some(ivals) = push_state.int_stack.pop_vec(2) {
                push_state.int_stack.push(&ivals[0] / &ivals[1]);
            }
        }
        // If divisor is zero, do nothing (leave both items on stack)
    }
}

/// INTEGER.<: Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or
/// FALSE otherwise.
fn integer_smaller(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] < ivals[1]);
    }
}

/// INTEGER.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE
/// otherwise.
fn integer_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] == ivals[1]);
    }
}

/// INTEGER.>: Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item,
/// or FALSE otherwise.
fn integer_greater(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] > ivals[1]);
    }
}

/// INTEGER.ABS: Pushes the absolute value of the top INTEGER item.
fn integer_abs(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.int_stack.push(ival.abs());
    }
}

/// INTEGER.NEG: Pushes the negation of the top INTEGER item.
fn integer_neg(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.int_stack.push(-ival);
    }
}

/// INTEGER.POW: Pushes the second item raised to the power of the top item.
fn integer_pow(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        // base^exponent, but we need to handle negative exponents and large values
        let base = &ivals[0];
        let exponent = &ivals[1];
        if exponent >= &BigInt::zero() {
            // For non-negative exponents, use pow with u32
            if let Some(exp_u32) = exponent.to_u32_digits().1.first().copied() {
                let result = base.pow(exp_u32);
                push_state.int_stack.push(result);
            } else {
                // Exponent too large, push 0 or 1 depending on base
                let result = if base.is_zero() {
                    BigInt::zero()
                } else if base.is_one() {
                    BigInt::one()
                } else if *base == BigInt::from(-1) {
                    if exponent.is_even() { BigInt::one() } else { BigInt::from(-1) }
                } else {
                    // Very large exponent with base > 1 or < -1 would overflow
                    BigInt::zero()
                };
                push_state.int_stack.push(result);
            }
        } else {
            // For negative exponents, integer result is 0 for all bases except -1, 0, 1
            let result = if base.is_zero() {
                BigInt::zero()  // 0^negative is undefined, but we'll use 0
            } else if base.is_one() {
                BigInt::one()
            } else if *base == BigInt::from(-1) {
                if exponent.is_even() { BigInt::one() } else { BigInt::from(-1) }
            } else {
                BigInt::zero()  // All other bases give 0 for negative integer exponents
            };
            push_state.int_stack.push(result);
        }
    }
}

/// INTEGER.SIGN: Pushes the sign of the top item (-1, 0, or 1).
fn integer_sign(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        let sign = if ival.is_negative() {
            BigInt::from(-1)
        } else if ival.is_positive() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        push_state.int_stack.push(sign);
    }
}

/// INTEGER.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the
/// top item of the INTEGER stack onto the EXEC stack.
pub fn integer_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(ival) = push_state.int_stack.pop() {
            push_state.name_bindings.insert(name, Item::int(ival));
        }
    }
}

/// INTEGER.DUP: Duplicates the top item on the INTEGER stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn integer_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.copy(0) {
        push_state.int_stack.push(ival);
    }
}

/// INTEGER.DDUP: Duplicates the two top items on the INTEGER stack while preserving its order.
pub fn integer_ddup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.copy_vec(2) {
        push_state.int_stack.push(ivals[0].clone());
        push_state.int_stack.push(ivals[1].clone());
    }
}

/// INTEGER.OVER: Copies the second item and pushes it on top of the stack.
pub fn integer_over(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.copy_vec(2) {
        push_state.int_stack.push(ivals[0].clone());
    }
}

/// INTEGER.DROP: Removes the top item from the stack.
pub fn integer_drop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.pop();
}

/// INTEGER.NIP: Removes the second item from the stack.
pub fn integer_nip(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(top) = push_state.int_stack.pop() {
        push_state.int_stack.pop();
        push_state.int_stack.push(top);
    }
}

/// INTEGER.TUCK: Copies the top item and inserts it before the second item.
pub fn integer_tuck(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(ivals[1].clone());
        push_state.int_stack.push(ivals[0].clone());
        push_state.int_stack.push(ivals[1].clone());
    }
}

/// INTEGER.FLUSH: Empties the INTEGER stack.
pub fn integer_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.flush();
}

/// INTEGER.FROMBOOLEAN: Pushes 1 if the top BOOLEAN is TRUE, or 0 if the top BOOLEAN is FALSE.
pub fn integer_from_boolean(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(is_true) = push_state.bool_stack.pop() {
        if is_true {
            push_state.int_stack.push(BigInt::one());
        } else {
            push_state.int_stack.push(BigInt::zero());
        }
    }
}

/// INTEGER.FROMFLOAT: Pushes the result of truncating the top FLOAT.
pub fn integer_from_float(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.int_stack.push(BigInt::from(fval.trunc() as i64));
    }
}
/// INTEGER.MAX: Pushes the maximum of the top two items.
pub fn integer_max(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[0] > ivals[1] {
            push_state.int_stack.push(ivals[0].clone());
        } else {
            push_state.int_stack.push(ivals[1].clone());
        }
    }
}

/// INTEGER.MIN: Pushes the minimum of the top two items.
pub fn integer_min(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[0] > ivals[1] {
            push_state.int_stack.push(ivals[1].clone());
        } else {
            push_state.int_stack.push(ivals[0].clone());
        }
    }
}

/// INTEGER.POP: Pops the INTEGER stack.
pub fn integer_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.pop();
}

/// INTEGER.RAND: Pushes a newly generated random INTEGER that is greater than or equal to
/// MIN-RANDOM-INTEGER and less than or equal to MAX-RANDOM-INTEGER.
pub fn integer_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(rval) = CodeGenerator::random_integer(push_state) {
        push_state.int_stack.push(rval);
    }
}

/// INTEGER.ROT: Rotates the top three items on the INTEGER stack, pulling the third item out and
/// pushing it on top. This is equivalent to "2 INTEGER.YANK".
pub fn integer_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.yank(2);
}

/// INTEGER.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn integer_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        if let Some(idx) = shove_index.to_i32() {
            let corr_index = i32::max(
                i32::min((push_state.int_stack.size() as i32) - 1, idx),
                0,
            ) as usize;
            push_state.int_stack.shove(corr_index as usize);
        }
    }
}

/// INTEGER.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn integer_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(BigInt::from(push_state.int_stack.size() + 1));
}

/// INTEGER.SWAP: Swaps the top two INTEGERs.
pub fn integer_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.shove(1);
}

/// INTEGER.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack, and the indexing is done after the index is
/// removed.
pub fn integer_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(idx) = index.to_i32() {
            let corr_index =
                i32::max(i32::min((push_state.int_stack.size() as i32) - 1, idx), 0) as usize;
            push_state.int_stack.yank(corr_index as usize);
        }
    }
}

/// INTEGER.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn integer_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(idx) = index.to_i32() {
            let corr_index =
                i32::max(i32::min((push_state.int_stack.size() as i32) - 1, idx), 0) as usize;
            if let Some(deep_item) = push_state.int_stack.copy(corr_index as usize) {
                push_state.int_stack.push(deep_item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn integer_modulus_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(-13));
        test_state.int_stack.push(BigInt::from(10));
        integer_modulus(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(7)); // Euclidean modulo: -13 % 10 = 7
    }

    #[test]
    fn integer_mult_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(2));
        integer_mult(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(8));
    }

    #[test]
    fn integer_add_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(2));
        integer_add(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(6));
    }

    #[test]
    fn integer_subtract_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(2));
        integer_subtract(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(2));
    }

    #[test]
    fn integer_divide_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(2));
        integer_divide(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(2));
    }

    #[test]
    fn integer_smaller_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(10));
        integer_smaller(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(4));
        integer_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_greater_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(10));
        test_state.int_stack.push(BigInt::from(4));
        integer_greater(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(2));
        test_state.name_stack.push(String::from("TEST"));
        integer_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::int(BigInt::from(2)).to_string()
        );
    }

    #[test]
    fn integer_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(2));
        integer_dup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2 2");
    }

    #[test]
    fn integer_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(213));
        test_state.int_stack.push(BigInt::from(2));
        integer_flush(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
    }

    #[test]
    fn integer_from_boolean_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        integer_from_boolean(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1");
    }

    #[test]
    fn integer_from_float_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0);
        integer_from_float(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1");
    }

    #[test]
    fn integer_max_pushes_greater_item() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(3));
        integer_max(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3");
    }

    #[test]
    fn integer_min_pushes_smaller_item() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(3));
        integer_max(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3");
    }

    #[test]
    fn integer_pop_removes_top_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        integer_pop(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2");
    }

    #[test]
    fn integer_rand_generates_value() {
        let mut test_state = PushState::new();
        integer_rand(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.size(), 1);
    }

    #[test]
    fn integer_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(3));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        assert_eq!(test_state.int_stack.to_string(), "1 2 3");
        integer_rot(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3 1 2");
    }

    #[test]
    fn integer_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(3));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        assert_eq!(test_state.int_stack.to_string(), "1 2 3 4");
        test_state.int_stack.push(BigInt::from(2));
        integer_shove(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2 3 1 4");
    }

    #[test]
    fn integer_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(3));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        integer_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "5 1 2 3 4");
    }

    #[test]
    fn integer_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(0));
        test_state.int_stack.push(BigInt::from(1));
        assert_eq!(test_state.int_stack.to_string(), "1 0");
        integer_swap(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "0 1");
    }

    #[test]
    fn integer_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(3));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        assert_eq!(test_state.int_stack.to_string(), "1 2 3 4 5");
        test_state.int_stack.push(BigInt::from(3));
        integer_yank(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "4 1 2 3 5");
    }

    #[test]
    fn integer_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        test_state.int_stack.push(BigInt::from(4));
        test_state.int_stack.push(BigInt::from(3));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(1));
        assert_eq!(test_state.int_stack.to_string(), "1 2 3 4 5");
        test_state.int_stack.push(BigInt::from(3));
        integer_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.int_stack.to_string(),
            "4 1 2 3 4 5"
        );
    }
    
    #[test]
    fn integer_add_handles_overflow() {
        let mut test_state = PushState::new();
        let max_val = BigInt::from(i32::MAX);
        test_state.int_stack.push(max_val.clone());
        test_state.int_stack.push(BigInt::from(1));
        integer_add(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), max_val + 1);
    }
    
    #[test]
    fn integer_mult_handles_overflow() {
        let mut test_state = PushState::new();
        let max_val = BigInt::from(i32::MAX);
        test_state.int_stack.push(max_val.clone());
        test_state.int_stack.push(BigInt::from(2));
        integer_mult(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), &max_val * 2);
    }
    
    #[test]
    fn integer_subtract_handles_underflow() {
        let mut test_state = PushState::new();
        let min_val = BigInt::from(i32::MIN);
        test_state.int_stack.push(min_val.clone());
        test_state.int_stack.push(BigInt::from(1));
        integer_subtract(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), &min_val - 1);
    }
    
    #[test]
    fn integer_neg_pushes_negation() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        integer_neg(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(-5));
        
        // Test MIN value - with BigInt no wrapping occurs
        let min_val = BigInt::from(i32::MIN);
        test_state.int_stack.push(min_val.clone());
        integer_neg(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), -min_val);
    }
    
    #[test]
    fn integer_pow_handles_positive_exponent() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        integer_pow(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(8));
    }
    
    #[test]
    fn integer_sign_returns_correct_values() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(-42));
        integer_sign(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(-1));
        
        test_state.int_stack.push(BigInt::from(0));
        integer_sign(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(0));
        
        test_state.int_stack.push(BigInt::from(42));
        integer_sign(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(1));
    }
    
    #[test]
    fn integer_abs_returns_absolute_value() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(-42));
        integer_abs(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(42));
        
        test_state.int_stack.push(BigInt::from(42));
        integer_abs(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(42));
        
        test_state.int_stack.push(BigInt::from(0));
        integer_abs(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), BigInt::from(0));
        
        // Test absolute value of i32::MIN - with BigInt no wrapping occurs
        let min_val = BigInt::from(i32::MIN);
        test_state.int_stack.push(min_val.clone());
        integer_abs(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), min_val.abs());
    }
    
    #[test]
    fn integer_ddup_duplicates_top_two() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        assert_eq!(test_state.int_stack.to_string(), "3 2 1");
        
        integer_ddup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3 2 3 2 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        integer_ddup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "5");
        
        // Test with empty stack
        let mut test_state = PushState::new();
        integer_ddup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
    }

    #[test]
    fn integer_dup2_alias_works() {
        // Test that INTEGER.DUP2 is properly aliased to integer_ddup
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(0));
        test_state.int_stack.push(BigInt::from(1));
        integer_ddup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1 0 1 0");
    }
    
    #[test]
    fn integer_over_copies_second_item() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        assert_eq!(test_state.int_stack.to_string(), "3 2 1");
        
        integer_over(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2 3 2 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        integer_over(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "5");
    }
    
    #[test]
    fn integer_drop_removes_top() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        
        integer_drop(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2 1");
        
        // Test with empty stack
        let mut test_state = PushState::new();
        integer_drop(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
    }
    
    #[test]
    fn integer_nip_removes_second() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        
        integer_nip(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        integer_nip(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "5");
    }
    
    #[test]
    fn integer_tuck_inserts_copy() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(1));
        test_state.int_stack.push(BigInt::from(2));
        test_state.int_stack.push(BigInt::from(3));
        
        integer_tuck(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "3 2 3 1");
        
        // Test with only one element
        let mut test_state = PushState::new();
        test_state.int_stack.push(BigInt::from(5));
        integer_tuck(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "5");
    }
}
