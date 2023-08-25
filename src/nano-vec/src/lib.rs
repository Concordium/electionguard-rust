// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(non_camel_case_types)] //? TODO remove?
#![allow(non_snake_case)] //? TODO remove?
#![allow(dead_code)] //? TODO remove
#![allow(unused_imports)] //? TODO remove

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

    pub fn push(&mut self, t: T) -> Result<(), NanoVecError> {
        for refmut_opt_t in self.0.iter_mut() {
            if refmut_opt_t.is_none() {
                refmut_opt_t.replace(t);
                return Ok(());
            }
        }
        Err(NanoVecError::Full)
    }

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
}

impl<S, T, const CAPACITY: usize> FromIterator<S> for NanoVec<T, CAPACITY>
where
    S: Into<T>
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

impl<S, T, const N: usize, const CAPACITY: usize> From<[S; N]> for NanoVec<T, CAPACITY>
where
    S: Into<T>,
    // [(); {CAPACITY - N}]: Sized, // TODO: Waiting on #[cfg(generic_const_exprs)]
{
    /// Convert an array of `S` into a NanoVec<T> of equal or greater capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use nano_vec::NanoVec;
    /// assert_eq!(NanoVec::from([1, 2, 3]), vec![1, 2, 3]);
    /// ```
    fn from(s: [S; N]) -> Self {
        // TODO: Waiting on #[cfg(generic_const_exprs)]
        assert!(s.len() <= Self::CAPACITY, "Source array too large");

        Self::from_iter(s)
        // let a = S[0..]from_fn(|_ix| iter.next());
        // Self(a)
        // NanoVec(a.map(Into::into))
    }
}
