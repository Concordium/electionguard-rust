#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO
#![allow(unused_variables)] //? TODO
#![allow(unreachable_code)] //? TODO
#![allow(non_camel_case_types)] //? TODO

use crate::endian::*;
use crate::primitive_unsigned::*;
use crate::primitive_unsigned::PrimitiveType;

pub trait StorageOrganization {
    /// The allocation unit type, one of `u8`, `u16`, `u32`, `u64`, or `u128`.
    type T: PrimitiveType;

    /// The number of allocation array units. Total size (`size_of::<[T; N]>()`) should be exactly `(1 << ALIGN_L2)*N`.
    const N: usize;

    fn array_n() -> usize {
        Self::N
    }

    /// Allocation array type. Typically `[T; N]`, but could be some other types that implement
    /// `AsRef<[T; N]>` `AsMut<[T; N]>`<br/>
    /// `AsRef<[T]>` `AsMut<[T]>`<br/>
    /// `std::simd::Simd` through `to_array() -> [T; N]`, `as_array() -> &[T; N]` and `as_mut_array() -> &[T; N]`<br/>
    /// `std::simd::Simd` through `Index::index(&self, index: I) -> ...` and `Index::index_mut(&mut self, index: I) -> ...`<br/>
    type ArrayT;

    /// Typically this will be the same as the allocation unit type's alignment.<br/>
    /// `T::BITS` should be less than or equal to `(1 << (ALIGN_L2 + 3))`<br/>
    /// E.g. 0 for `u8`, 3 for `u64`.
    ///
    /// Compare to: `align_of::<[std::simd::Simd<T, N>]>()`
    ///
    const ALIGN_N: u32 = std::mem::align_of::<Self::T>() as u32;

    fn align_n() -> u32 {
        Self::ALIGN_N
    }

    /// The log<sub>2</sub> of the alignment of ArrayT.
    const ALIGN_L2: u32 = 0; //? TODO

    fn align_l2() -> u32 {
        1_u32 << Self::ALIGN_L2
    }

    /// Sequence order of the allocation array elements within the overall representation.
    fn elem_order() -> SequenceOrEndian {
        SequenceOrEndian::Sequence(SequenceOrder::Forward)
    }

    /// Endianness, order of bytes, within allocation elements.
    fn byte_order() -> ByteOrder {
        ByteOrder {
            absolute_endian: Endian::Little,
            relative_endian: RelativeEndian::Native,
        }
    }

    /// Sequence order of bits within each byte.
    fn bit_order() -> BitOrder {
        BitOrder::Forward
    }
}
