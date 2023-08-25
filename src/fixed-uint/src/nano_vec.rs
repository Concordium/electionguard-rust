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

/// A very small contiguous container, like `Vec` with a fixed capacity.
///
/// `T` is the element type.
///
/// Internal storage is simply an array of `[Option<T>; CAPACITY]`. This implies that:
///
/// 1. Most operations such as `len()`, `push()`, and `pop()` are O(N) or O(`CAPACITY`).
///
/// 2. The best types to use for this are those for which `size_of<Option<T>> == size_of<T>`.
/// Some examples are `std::ptr::NonNull` and the `std::num::NonZero*` family of types.
/// These types have the rustc built-in attribute `#[rustc_nonnull_optimization_guaranteed]`.
/// Unfortunately, this attribute "will never be stable", so you'll need to convert your own
/// types to and from these basic types manually.
///
/// Since this is the primary use of this type, a `is_compact()` const method is provided
/// to verify that is the case.
///
#[derive(Clone, Copy)]
pub struct NanoVec<T, const CAPACITY: usize>([Option<T>; CAPACITY]);

impl<T, const CAPACITY: usize> NanoVec<T, CAPACITY> {
    //type Inner = [Option<T>; CAPACITY];

    /// The maximum number of elements the container can store.
    /// This value is fixed, no reallocation is allowed.
    pub const CAPACITY: usize = CAPACITY;

    /// An instance of the empty container.
    pub const DEFAULT: Self = Self([Self::OPTION_T_NONE; CAPACITY]);
    const OPTION_T_NONE: Option<T> = None;

    // Returns true iff no space is wasted over a simple array of `[T; CAPACITY]`.
    #[must_use]
    #[inline]
    pub const fn is_compact() -> bool {
        std::mem::size_of::<[Option<T>; CAPACITY]>() <= std::mem::size_of::<[T; CAPACITY]>()
    }

    // The maximum number of elements the container can store.
    // This value is fixed, no reallocation is allowed.
    #[must_use]
    #[inline]
    pub const fn capacity() -> usize {
        CAPACITY
    }

    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self::DEFAULT
    }

    /// Returns an `Option<&T>` possibly referring to the element at the specified index.
    #[inline]
    pub const fn opt_ref_at(&self, ix: usize) -> Option<&T> {
        if ix < Self::CAPACITY {
            self.0[ix].as_ref()
        } else {
            None
        }
    }

    /// Returns an `Option<&mut T>` possibly referring to the element at the specified
    /// index.
    #[inline]
    pub fn opt_mut_at(&mut self, ix: usize) -> Option<&mut T> {
        if ix < Self::CAPACITY {
            self.0[ix].as_mut()
        } else {
            None
        }
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

    /// Returns the length of the stored sequence.
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

    /// Shortens the the stored sequence.
    /// Has no effect if `resulting_len` is greater than or equal to the current length.
    pub fn truncate(&mut self, resulting_len: usize) {
        for opt_elem in self.0.iter_mut().skip(resulting_len) {
            if opt_elem.is_some() {
                *opt_elem = None;
            } else {
                break;
            }
        }
    }
}

impl<T, const CAPACITY: usize> NanoVec<T, CAPACITY>
where
    T: Copy,
{
    //? TODO pub fn push_within_capacity(&mut self, value: T) -> Result<(), T>
    //? TODO pub fn insert(&mut self, index: usize, element: T)
    //? TODO pub fn remove(&mut self, index: usize) -> T
    //? TODO retain?
    //? TODO retain_mut?
    //? TODO dedup_by_key?
    //? TODO dedup_by?
    //? TODO pub fn clear(&mut self)
    //? TODO pub fn iter(&self) -> Iter<'_, T>
    //? TODO pub fn iter_mut(&mut self) -> IterMut<'_, T>
    //? TODO pub fn as_mut(&mut self) -> Option<&mut T>
    //? TODO
}

impl<S, T, const CAPACITY: usize> FromIterator<S> for NanoVec<T, CAPACITY>
where
    S: Into<T>,
    T: std::fmt::Debug,
{
    /// Creates a `NanoVec<T, CAPACITY>` from an iterator over `T`.
    /// Note that at most only `CAPACITY` elements will be requested from the source iterator.
    fn from_iter<IIS>(ii: IIS) -> Self
    where
        IIS: IntoIterator<Item = S>,
    {
        // TODO Would be nice to have a  `const` version of this someday, but currently:
        // error[E0015]: cannot call non-const fn `<IIS as IntoIterator>::into_iter` in constant functions
        // `std::iter::Fuse` uses an `Option` internally that Rust can't yet drop in const context.

        //? TODO See example implementation of `from_iter_fallible`
        use std::array::from_fn;

        let mut iter = ii.into_iter().map(Into::into).fuse();
        let a = from_fn(|_ix| iter.next());
        Self(a)
    }
}

/*
impl<T, const N: usize, const CAPACITY: usize> From<[T; N]> for NanoVec<T, CAPACITY>
{
    /// Convert an array of `T` into a NanoVec<T> of exactly that capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(NanoVec::from([1, 2, 3]), vec![1, 2, 3]);
    /// ```
    fn from(a: [T; N]) -> Self {
        NanoVec(a.map(Into::into))
    }
}
*/

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
            #[allow(clippy::unwrap_used)]
            mod [< test_NanoVec_ $elem_t >] {
                use super::*;

                use std::num::$elem_t;

                const ELEM_T_STR: &str = stringify!($elem_t);
                const PRECONV_T_STR: &str = stringify!($preconv_t);

                type ElemT = $elem_t;
                type PreconvT = $preconv_t;

                fn usize_to_elem(u: usize) -> ElemT {
                    let u128_u: u128 = u.try_into().unwrap();

                    let u128_elem_min: u128 = 1;

                    let preconv_elem_max: PreconvT = ElemT::MAX.try_into().unwrap();
                    let u128_elem_max: u128 = preconv_elem_max.try_into().unwrap_or(u128::MAX);

                    let u128_elem_diff = u128_elem_max - u128_elem_min;

                    let u128_elem = u128_elem_min + u128_u%u128_elem_diff;

                    let preconv_elem: PreconvT = u128_elem.try_into().unwrap();
                    let elem: ElemT = preconv_elem.try_into().unwrap();

                    outln!("usize_to_elem({u}) -> {elem}: {ELEM_T_STR}");

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
                use super::*;

                const CAPACITY: usize = $capacity;

                type NanoVecT = NanoVec<ElemT, CAPACITY>;

                #[test]
                fn test_construction() {
                    outln!("\n================================= NanoVec<{ELEM_T_STR}, {CAPACITY}>");
                    outln!("type PreconvT = {PRECONV_T_STR}");

                    const_assert_eq!(NanoVecT::capacity(), CAPACITY);
                    const_assert_eq!(NanoVecT::CAPACITY, CAPACITY);

                    let init_seq = [
                        usize_to_elem(0),
                        usize_to_elem(1),
                        usize_to_elem(2),
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
                            Some(usize_to_elem(ix))
                        } else {
                            None
                        };

                        assert_eq!(nv.opt_ref_at(ix).copied(), expected_opt_elem);
                    }

                    for ix in 0..CAPACITY + 1 {
                        if let Some(mut_elem) = nv.opt_mut_at(ix) {
                            *mut_elem = usize_to_elem(ix + 100);
                        }
                    }

                    for ix in 0..CAPACITY + 1 {
                        outln!(
                            "nv.opt_ref_at({ix}) = {:?}",
                            nv.opt_ref_at({ix}).copied()
                        );

                        let expected_opt_elem = if ix < expected_len {
                            Some(usize_to_elem(ix + 100))
                        } else {
                            None
                        };

                        assert_eq!(nv.opt_ref_at(ix).copied(), expected_opt_elem);
                    }
                } // fn test_construction()

                // Test truncate()
                #[test]
                fn test_truncate() {
                    outln!("\n================================= NanoVec<{ELEM_T_STR}, {CAPACITY}>");
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
                    outln!("\n================================= NanoVec<{ELEM_T_STR}, {CAPACITY}>");
                    outln!("type PreconvT = {PRECONV_T_STR}");

                    let mut nv = NanoVecT::DEFAULT;

                    let mut expected_len: usize = 0;

                    for ix in 0..=CAPACITY {
                        outln!("------------------------ ix = {ix}");

                        let len = nv.len();
                        outln!("nv.len()) -> {len}");
                        assert_eq!(len, expected_len);

                        let elem = usize_to_elem(ix);
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
                            let elem = usize_to_elem(pop_n);
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
test_cases!(NonZeroI16, i16);
test_cases!(NonZeroI32, i32);
test_cases!(NonZeroI64, i64);
test_cases!(NonZeroI128, i128);
test_cases!(NonZeroIsize, isize);
