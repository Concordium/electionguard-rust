// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(non_camel_case_types)] //? TODO?
#![allow(non_snake_case)] //? TODO?
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO

use std::convert::{From, Into};
use std::default::Default;
use std::marker::PhantomData;
use std::mem::{align_of, size_of, size_of_val};

use static_assertions::*;

// Note the use of braces rather than parentheses.
custom_error::custom_error! {
    #[derive(PartialEq, Eq)]
    pub NanoVecError
    Full = "Container is full",
    Empty = "Container is empty",
}

/// A very small contiguous container, like `Vec` but without reallocation.
///
/// `NZ` is the storage type, one of the `NonZero*` types for which
/// `size_of<NZ> == size_of<Option<NZ>>`.
///
/// `T` is the element type.
///
/// Internal storage is simply an array of `[Option<T>; N]`. This implies that:
///
/// 1. Most operations such as `len()`, `push()`, and `pop()` are O(N) or O(`MAX_N`).
///
/// 2. The best types to use for this are those for which `size_of<NZ> == size_of<Option<NZ>>`.
/// These types have the rustc built-in attribute `#[rustc_nonnull_optimization_guaranteed]`.
/// Some examples are `std::ptr::NonNull` and the `std::num::NonZeroU8` family of types.
///
#[derive(Clone, Copy)]
pub struct NanoVec<T, const MAX_N: usize>([Option<T>; MAX_N])
where
    T: Clone + Copy;

impl<T, const MAX_N: usize> NanoVec<T, MAX_N>
where
    T: Clone + Copy,
{
    pub const MAX_CAPACITY: usize = MAX_N;
    pub const DEFAULT: Self = Self([None; MAX_N]);

    #[must_use]
    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub fn push(&mut self, t: T) -> Result<(), NanoVecError> {
        for refmut_opt_t in self.0.iter_mut() {
            if refmut_opt_t.is_none() {
                refmut_opt_t.replace(t);
                return Ok(());
            }
        }
        Err(NanoVecError::Full)
    }

    #[must_use]
    pub fn pop(&mut self) -> Result<T, NanoVecError> {
        for refmut_opt_t in self.0.iter_mut().rev() {
            if refmut_opt_t.is_some() {
                return Ok(refmut_opt_t.take().unwrap());
            }
        }
        Err(NanoVecError::Empty)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let mut n = 0usize;
        for opt_nz in self.0.iter() {
            if opt_nz.is_some() {
                n += 1;
            } else {
                break;
            }
        }
        n
    }
}

/*
trait NanoVecTrait {
    type Elem: Clone + Copy;
    const MAX_N: usize;
}

impl<TT, const MAX_NN: usize> NanoVecTrait for NanoVec<TT, MAX_NN>
where
    TT: Clone + Copy,
{
    type Elem = TT;
    const MAX_N: usize = MAX_NN;
}

trait NanoVecTrait2<NanoVecType> : NanoVecTrait {
    type Self_;
    const DEFAULT: NanoVecType;
}

impl<TT, const MAX_NN: usize> NanoVecTrait2<NanoVec<TT, MAX_NN>> for NanoVec<TT, MAX_NN>
where
    TT: Clone + Copy,
{
    type Self_ = NanoVec<TT, MAX_NN>;
    const DEFAULT: NanoVec<TT, MAX_NN> = NanoVec::new();
}
*/

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test_nanovec {
    use super::*;

    use std::num::{
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, 
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
    };

    macro_rules! eprintln {
        ($($arg:tt)*) => {{
            std::eprintln!($($arg)*);
        }};
    }

    #[test]
    fn test() {
        macro_rules! test_case {
            ($elem_t:path, $preconv_t:path, $count_t:path, $max_n:literal) => {{
                eprintln!(
                    "\n================================================ elem_t = {}, max_n = {}",
                    stringify!($elem_t),
                    $max_n
                );
                eprintln!("type ElemT = {}", stringify!($elem_t));
                eprintln!("type PreconvT = {}", stringify!($preconv_t));
                eprintln!("type CountT = {}", stringify!($count_t));

                type ElemT = $elem_t;
                type PreconvT = $preconv_t;
                type CountT = $count_t;

                assert_eq!(NanoVec::<$elem_t, $max_n>::MAX_CAPACITY, $max_n);

                let elem_val_min: CountT = CountT::try_from(1).unwrap();
                let elem_val_max: CountT =
                    CountT::try_from(PreconvT::try_from(ElemT::MAX).unwrap()).unwrap();
                let elem_val_diff: CountT = elem_val_max - elem_val_min;

                let count_to_elem = |n: CountT| -> ElemT {
                    //eprintln!("count_to_elem(n: {} = {n}) -> ", stringify!($count_t));

                    let elem_count = elem_val_min + n % elem_val_diff;
                    //eprintln!("elem_count = {elem_count}: {}", stringify!($count_t));

                    let preconv = PreconvT::try_from(elem_count).unwrap();
                    //eprintln!("preconv = {preconv}: {}", stringify!($preconv_t));

                    let elem = ElemT::try_from(preconv).unwrap();

                    eprintln!(
                        "count_to_elem(n: {} = {n}) -> {elem}: {}",
                        stringify!($count_t),
                        stringify!($elem_t)
                    );
                    elem
                };

                let mut nv: NanoVec<ElemT, $max_n> = NanoVec::DEFAULT;

                let push_count_max_n: CountT = $max_n;

                let push_count_first: CountT = 0;
                let push_count_last: CountT = push_count_max_n;

                let mut expected_len: usize = 0;

                for push_n in push_count_first..=push_count_last {
                    eprintln!("------------------------ push_n = {push_n}");

                    let len = nv.len();
                    eprintln!("nv.len()) -> {len}");
                    assert_eq!(len, expected_len);

                    let elem = count_to_elem(push_n);
                    eprintln!("elem = {elem}: {}", stringify!($elem_t));

                    let push_expected_result = if push_n < push_count_last {
                        Ok(())
                    } else {
                        Err(NanoVecError::Full)
                    };
                    let push_actual_result = nv.push(elem);
                    eprintln!("nv.push({elem}) -> {push_actual_result:?}");
                    assert_eq!(push_expected_result, push_actual_result);

                    if push_actual_result.is_ok() {
                        eprintln!(
                            "expected_len: {expected_len} -> {}",
                            expected_len.wrapping_add(1)
                        );
                        expected_len = expected_len.checked_add(1).unwrap()
                    }
                }

                for push_n in push_count_first..=push_count_last {
                    eprintln!("------------------------ push_n = {push_n}");

                    let len = nv.len();
                    eprintln!("nv.len()) -> {len}");
                    assert_eq!(len, expected_len);

                    let pop_expected_result = if push_n < push_count_last {
                        let pop_n = push_count_last
                            .checked_sub(1)
                            .unwrap()
                            .checked_sub(push_n)
                            .unwrap();

                        let elem = count_to_elem(pop_n);
                        eprintln!("elem = {elem}: {}", stringify!($elem_t));

                        Ok(elem)
                    } else {
                        Err(NanoVecError::Empty)
                    };

                    let pop_actual_result = nv.pop();
                    eprintln!("nv.pop() -> {pop_actual_result:?}");
                    assert_eq!(pop_expected_result, pop_actual_result);

                    if pop_actual_result.is_ok() {
                        eprintln!(
                            "expected_len: {expected_len} -> {}",
                            expected_len.wrapping_sub(1)
                        );
                        expected_len = expected_len.checked_sub(1).unwrap()
                    }
                }

                let len = nv.len();
                eprintln!("nv.len()) -> {len}");
                assert_eq!(len, expected_len);
            }};
        }

        // elem_t, preconv_t, count_t, max_n
        #[rustfmt::skip]
        // test_case!(          u8,   u8, usize,   0);
        // test_case!(          u8,   u8, usize, 255);
        // test_case!(         u16,  u16, usize,   2);
        // test_case!(         u32,  u32, usize,   2);
        // test_case!(         u64,  u64, usize,   2);
        // test_case!(        u128, u128,  u128,   2);
        // test_case!(   NonZeroU8,   u8, usize,   2);
        // test_case!(  NonZeroU16,  u16, usize,   2);
        // test_case!(  NonZeroU32,  u32, usize,   2);
        // test_case!(  NonZeroU64,  u64, usize,   2);
        // test_case!( NonZeroU128, u128, u128,   2);
        // test_case!(   NonZeroI8,   i8, usize,   2);
        // test_case!(  NonZeroI16,  i16, usize,   2);
        // test_case!(  NonZeroI32,  i32, usize,   2);
        // test_case!(  NonZeroI64,  i64, usize,   2);
        test_case!(NonZeroI128,  i128, u128,   2);
    }

    #[test]
    fn test2() {
        type T = u8;
        const DEFAULT_NV: NanoVec<T, 2> = NanoVec::new();
        let mut nv = DEFAULT_NV;
        assert_eq!(nv.len(), 0);

        assert_eq!(nv.push(0), Ok(()));
        assert_eq!(nv.len(), 1);

        assert_eq!(nv.push(1), Ok(()));
        assert_eq!(nv.len(), 2);

        assert_eq!(nv.push(2), Err(NanoVecError::Full));
        assert_eq!(nv.len(), 2);

        assert_eq!(nv.pop(), Ok(1));
        assert_eq!(nv.len(), 1);

        assert_eq!(nv.pop(), Ok(0));
        assert_eq!(nv.len(), 0);

        assert_eq!(nv.pop(), Err(NanoVecError::Empty));
    }
}

/*
/// Identifies instances of std::num::NonZero*
trait NonZeroPrimitive: Clone + Copy {}
impl NonZeroPrimitive for std::num::NonZeroU8 {}
impl NonZeroPrimitive for std::num::NonZeroU16 {}
impl NonZeroPrimitive for std::num::NonZeroU32 {}
impl NonZeroPrimitive for std::num::NonZeroU64 {}
impl NonZeroPrimitive for std::num::NonZeroU128 {}
impl NonZeroPrimitive for std::num::NonZeroUsize {}
impl NonZeroPrimitive for std::num::NonZeroI8 {}
impl NonZeroPrimitive for std::num::NonZeroI16 {}
impl NonZeroPrimitive for std::num::NonZeroI32 {}
impl NonZeroPrimitive for std::num::NonZeroI64 {}
impl NonZeroPrimitive for std::num::NonZeroI128 {}
impl NonZeroPrimitive for std::num::NonZeroIsize {}

/// A very small contiguous container, like `Vec` but without reallocation.
///
/// `NZ` is the storage type, one of the `NonZero*` types for which
/// `size_of<NZ> == size_of<Option<NZ>>`.
///
/// `T` is the element type, infallibly convertible between `NZ`.
///
#[derive(Clone, Copy)]
pub struct NanoVec<NZ, T, const N: usize>([Option<NZ>; N], PhantomData<fn(T) -> T>)
where
    NZ: Into<T> + Clone + Copy + NonZeroPrimitive,
    T: Into<NZ> + Clone + Copy;

impl<NZ, T, const N: usize> NanoVec<NZ, T, N>
where
    NZ: Into<T> + Clone + Copy + NonZeroPrimitive,
    T: Into<NZ> + Clone + Copy,
{
    #[must_use]
    pub const fn new() -> Self {
        Self([Option::<NZ>::None; N], PhantomData)
    }

    #[must_use]
    pub fn push(&mut self, elem: T) -> Result<(), NanoVecError> {
        for mutref_opt_nz in self.0.iter_mut() {
            if mutref_opt_nz.is_none() {
                *mutref_opt_nz = Some(elem.into());
                return Ok(());
            }
        }
        Err(NanoVecError::Full)
    }

    #[must_use]
    pub fn pop(&mut self) -> Result<T, NanoVecError> {
        for mutref_opt_nz in self.0.iter_mut().rev() {
            if let Some(nz) = *mutref_opt_nz {
                *mutref_opt_nz = None;
                return Ok(nz.into());
            }
        }
        Err(NanoVecError::Empty)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let mut n = 0usize;
        for opt_nz in self.0.iter() {
            if opt_nz.is_some() {
                n += 1;
            } else {
                break;
            }
        }
        n
    }
}
/*
// Copy trait must be implmented manually because of the PhantomData.
impl<S, T, const N: usize> Copy for NanoVec<S, T, N> {}

// Clone trait must be implmented manually because of the PhantomData.
impl<S, T, const N: usize> Clone for NanoVec<S, T, N> {
    fn clone(&self) -> Self {
        *self
    }
}
*/
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test_nanovec {
    use super::*;

    use std::num::NonZeroU8;
    #[derive(Clone, Copy, Debug)]
    struct T(NonZeroU8);

    impl From<NonZeroU8> for T {
        fn from(nz: NonZeroU8) -> Self {
            T(nz)
        }
    }

    impl From<T> for NonZeroU8 {
        fn from(t: T) -> Self {
            t.0
        }
    }

    #[test]
    fn test() {
        let mut nv = NanoVec::<NonZeroU8, T, 2>::new();
        assert_eq!(nv.len(), 0);
        nv.push(T(1u8.try_into().unwrap())).expect("");
        assert_eq!(nv.len(), 1);
        nv.push(T(2u8.try_into().unwrap())).expect("");
        assert_eq!(nv.len(), 2);
        nv.push(T(2u8.try_into().unwrap())).expect_err("");
        assert_eq!(nv.len(), 2);
        nv.pop().expect("");
        assert_eq!(nv.len(), 1);
        nv.pop().expect("");
        assert_eq!(nv.len(), 0);
        nv.pop().expect_err("");
        assert_eq!(nv.len(), 0);
    }
}
*/
