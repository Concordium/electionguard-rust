// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

#![allow(clippy::unwrap_used)]

#![allow(non_camel_case_types)] //? TODO remove?
#![allow(non_snake_case)] //? TODO remove?
#![allow(dead_code)] //? TODO remove
#![allow(unused_imports)] //? TODO remove

// use std::convert::{From, Into};
// use std::default::Default;
// use std::marker::PhantomData;
// use std::mem::{align_of, size_of, size_of_val};

use static_assertions::*;

#[cfg(test)]
macro_rules! outln {
    ($($arg:tt)*) => {{
        std::eprintln!($($arg)*);
    }};
}

macro_rules! test_cases {
    ($elem_t:path, $preconv_t:path) => {
        paste::paste! {
            #[cfg(test)]
            mod [< test_elem_ $elem_t >] {
                use super::*;

                use std::num::$elem_t;

                type ElemT = $elem_t;
                const ELEM_T_STR: &str = stringify!($elem_t);

                type PreconvT = $preconv_t;
                const PRECONV_T_STR: &str = stringify!($preconv_t);

                fn usize_to_elemt(u: usize) -> ElemT {
                    let u128_u: u128 = u.try_into().unwrap();

                    let u128_elem_min: u128 = 1;

                    let preconv_elem_max: PreconvT = ElemT::MAX.try_into().unwrap();
                    let u128_elem_max: u128 = preconv_elem_max.try_into().unwrap_or(u128::MAX);

                    let u128_elem_diff = u128_elem_max - u128_elem_min;

                    let u128_elem = u128_elem_min + u128_u%u128_elem_diff;

                    let preconv_elem: PreconvT = u128_elem.try_into().unwrap();
                    let elem: ElemT = preconv_elem.try_into().unwrap();

                    outln!("usize_to_elemt({u}) -> {elem}: {ELEM_T_STR}");

                    elem
                }

                test_cases!(@with_capacity, 0);
                test_cases!(@with_capacity, 1);
                test_cases!(@with_capacity, 5);
            } // mod [< test_nanovec_ $elem_t >]
        } // paste
    };

    (@with_capacity, $capacity:literal) => {
        paste::paste! {
            mod [< capacity_ $capacity >] {
                use crate::nano_vec::{NanoVec, NanoVecError};
                use super::*;

                const CAPACITY: usize = $capacity;

                type NanoVecT = NanoVec<ElemT, CAPACITY>;

                #[test]
                fn test_construction() {
                    outln!("\n================================= test_construction() for NanoVec<{ELEM_T_STR}, {CAPACITY}>");
                    outln!("type PreconvT = {PRECONV_T_STR}");

                    const_assert_eq!(NanoVecT::capacity(), CAPACITY);
                    const_assert_eq!(NanoVecT::CAPACITY, CAPACITY);

                    let init_seq = [
                        usize_to_elemt(0),
                        usize_to_elemt(1),
                        usize_to_elemt(2),
                    ];

                    let expected_len = init_seq.len().min(CAPACITY);

                    let mut nv = NanoVecT::from_iter(init_seq);
                    assert_eq!(nv.len(), expected_len);

                    for ix in 0..CAPACITY + 1 {
                        outln!(
                            "nv.opt_ref_at({ix}) = {:?}",
                            nv.opt_ref_at({ix}).copied()
                        );

                        let expected_opt_elem = if ix < expected_len {
                            Some(usize_to_elemt(ix))
                        } else {
                            None
                        };

                        assert_eq!(nv.opt_ref_at(ix).copied(), expected_opt_elem);
                    }

                    for ix in 0..CAPACITY + 1 {
                        if let Some(mut_elem) = nv.opt_mut_at(ix) {
                            *mut_elem = usize_to_elemt(ix + 100);
                        }
                    }

                    for ix in 0..CAPACITY + 1 {
                        outln!(
                            "nv.opt_ref_at({ix}) = {:?}",
                            nv.opt_ref_at({ix}).copied()
                        );

                        let expected_opt_elem = if ix < expected_len {
                            Some(usize_to_elemt(ix + 100))
                        } else {
                            None
                        };

                        assert_eq!(nv.opt_ref_at(ix).copied(), expected_opt_elem);
                    }
                } // fn test_construction()

                // Test truncate()
                #[test]
                fn test_truncate() {
                    outln!("\n================================= test_truncate() for NanoVec<{ELEM_T_STR}, {CAPACITY}>");
                    outln!("type PreconvT = {PRECONV_T_STR}");

                    let default_elem = ElemT::try_from(1).unwrap();

                    if 1 <= CAPACITY {
                        let mut nv = NanoVecT::DEFAULT;
                        let mut expected_len: usize = 0;

                        assert_eq!(nv.len(), expected_len);

                        nv.truncate(expected_len);
                        assert_eq!(nv.len(), expected_len);

                        nv.push(default_elem).unwrap();
                        expected_len += 1;
                        assert_eq!(nv.len(), expected_len);

                        nv.truncate(CAPACITY + 1);
                        assert_eq!(nv.len(), expected_len);

                        nv.truncate(CAPACITY);
                        assert_eq!(nv.len(), expected_len);

                        nv.truncate(expected_len);
                        assert_eq!(nv.len(), expected_len);

                        if 2 <= CAPACITY {
                            nv.push(default_elem).unwrap();
                            expected_len += 1;
                            assert_eq!(nv.len(), expected_len);

                            nv.truncate(expected_len);
                            assert_eq!(nv.len(), expected_len);

                            nv.truncate(1);
                            expected_len = 1;
                            assert_eq!(nv.len(), expected_len);
                        }

                        nv.truncate(0);
                        expected_len = 0;
                        assert_eq!(nv.len(), expected_len);
                    }
                } // fn test_truncate()

                #[test]
                fn test_push_pop() {
                    outln!("\n================================= test_push_pop() for NanoVec<{ELEM_T_STR}, {CAPACITY}>");
                    outln!("type PreconvT = {PRECONV_T_STR}");

                    let mut nv = NanoVecT::DEFAULT;

                    let mut expected_len: usize = 0;

                    for ix in 0..=CAPACITY {
                        outln!("------------------------ ix = {ix}");

                        let len = nv.len();
                        outln!("nv.len()) -> {len}");
                        assert_eq!(len, expected_len);

                        let elem = usize_to_elemt(ix);
                        outln!("elem = {elem}: {ELEM_T_STR}");

                        let push_expected_result = if ix < CAPACITY {
                            Ok(())
                        } else {
                            Err(NanoVecError::Full)
                        };
                        let push_actual_result = nv.push(elem);
                        outln!("nv.push({elem}) -> {push_actual_result:?}");
                        assert_eq!(push_expected_result, push_actual_result);

                        if push_actual_result.is_ok() {
                            outln!(
                                "expected_len: {expected_len} -> {}",
                                expected_len.wrapping_add(1)
                            );
                            expected_len = expected_len.checked_add(1).unwrap()
                        }
                    }

                    for ix in 0..=CAPACITY {
                        outln!("------------------------ ix = {ix}");

                        let len = nv.len();
                        outln!("nv.len()) -> {len}");
                        assert_eq!(len, expected_len);

                        let pop_expected_result = if ix < CAPACITY {
                            let pop_n = CAPACITY.saturating_sub(1).saturating_sub(ix);
                            let elem = usize_to_elemt(pop_n);
                            outln!("expecting elem {elem}: {ELEM_T_STR}");
                            Ok(elem)
                        } else {
                            Err(NanoVecError::Empty)
                        };

                        let pop_actual_result = nv.pop();
                        outln!("nv.pop() -> {pop_actual_result:?}");
                        assert_eq!(pop_expected_result, pop_actual_result);

                        if pop_actual_result.is_ok() {
                            outln!(
                                "expected_len: {expected_len} -> {}",
                                expected_len.wrapping_sub(1)
                            );
                            expected_len = expected_len.checked_sub(1).unwrap()
                        }
                    }

                    let len = nv.len();
                    outln!("nv.len()) -> {len}");
                    assert_eq!(len, expected_len);
                } // fn test_push_pop()
            } // mod [< capacity_ $capacity >]
        } // paste!
    };
} // macro_rules!

// elem_t, preconv_t
test_cases!(NonZeroU8, u8);
test_cases!(NonZeroU16, u16);
test_cases!(NonZeroU32, u32);
test_cases!(NonZeroU64, u64);
test_cases!(NonZeroU128, u128);
test_cases!(NonZeroUsize, usize);
test_cases!(NonZeroI16, i16);
test_cases!(NonZeroI32, i32);
test_cases!(NonZeroI64, i64);
test_cases!(NonZeroI128, i128);
test_cases!(NonZeroIsize, isize);
