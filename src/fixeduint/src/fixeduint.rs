// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

use static_assertions::*;
use std::mem::size_of;
use std::ops::Mul;

use crate::bitvec_organization::*;
use crate::endian::*;
use crate::primitive_unsigned::*;
use crate::teprintln;

struct FixedSizeArrayOfUnsigned<ElemT, const ARRAY_N: usize>
where
    ElemT: PrimitiveType<PrimitiveType = ElemT>,
{
    a: [ElemT; ARRAY_N],
    elem_order: SequenceOrEndian,
    byte_order: ByteOrder,
}

impl<ElemT, const ARRAY_N: usize> StorageOrganization for FixedSizeArrayOfUnsigned<ElemT, ARRAY_N>
where
    ElemT: PrimitiveType<PrimitiveType = ElemT>,
{
    type T = ElemT;

    /// The number of allocation array units. Total size (`size_of::<[T; N]>()`) should be
    /// exactly `(1 << ALIGN_L2)*N`.
    const N: usize = ARRAY_N;

    type ArrayT = [ElemT; ARRAY_N];

    // As a large integer, we will stay in big-endian for now.
    fn elem_order() -> SequenceOrEndian {
        SequenceOrEndian::Endian(Endian::Big)
    }
}

impl<ElemT, const ARRAY_N: usize> FixedSizeArrayOfUnsigned<ElemT, ARRAY_N>
where
    ElemT: PrimitiveType<PrimitiveType = ElemT>,
{
    pub const ARRAY_N: usize = ARRAY_N;

    pub const ARRAY_ELEM_ALIGN: usize = ElemT::ALIGN;
    pub const ARRAY_ELEM_SIZE: usize = ElemT::SIZE;
    pub const ARRAY_ELEM_BITS: u32 = ElemT::BITS;

    /// Creates a fixed bitpattern such that the upper nibble of every byte is the
    /// element index and the lower nibble is the byte index within the element.
    /// For testing.
    ///
    /// For example:
    /// ```rust,ignore
    ///     FixedUint<ElemT = u128, ARRAY_N = 3> {
    ///         a: [
    ///             0: 0x2F2E2D2C2B2A29282726252423222120,
    ///             1: 0x1F1E1D1C1B1A19181716151413121110,
    ///             2: 0x0F0E0D0C0B0A09080706050403020100
    ///         ],
    ///         elem_order: Endian(Big) 'B',
    ///         byte_order: ByteOrder { absolute_endian: Little, relative_endian: Native },
    ///         bit_order: Forward
    ///     }
    /// ```
    ///
    pub fn make_fixed_bitpattern_array() -> <Self as StorageOrganization>::ArrayT {
        let is_elem_order_big_endian =
            matches!(Self::elem_order(), SequenceOrEndian::Endian(Endian::Big));

        //teprintln!("is_elem_order_big_endian: {}", is_elem_order_big_endian);//?

        std::array::from_fn(|mut elem_ix| {
            assert!(elem_ix < Self::ARRAY_N);

            if is_elem_order_big_endian && elem_ix <= Self::ARRAY_N {
                elem_ix = Self::ARRAY_N.saturating_sub(elem_ix + 1);
            }

            let elemix_nibble = elem_ix.min(15) as u8;

            let mut val = ElemT::ZERO;
            for byte_ix in 0..ElemT::SIZE {
                let shift_bits = byte_ix * 8;
                let by = (elemix_nibble << 4) | (byte_ix.min(15) as u8);
                val |= ElemT::from(by) << shift_bits;
            }

            val
        })
    }

    #[allow(clippy::let_and_return)]
    pub fn new_fixed_bitpattern() -> Self {
        //type ElemT = <Self as StorageOrganization>::T;
        //type ArrayT = [ElemT; ARRAY_ELEM_SIZE];
        //type ArrayT = [ElemT; Self::array_n()]; // <Self as StorageOrganization>::ArrayT;
        //ARRAY_ELEM_ALIGN

        let self_ = Self {
            a: Self::make_fixed_bitpattern_array(),
            elem_order: SequenceOrEndian::Endian(Endian::Big),
            byte_order: Self::byte_order(),
        };

        // target datalayout =
        // "e-"
        // "m:e-"
        // "p270:32:32-"
        // "p271:32:32-"
        // "p272:64:64-"
        // "i64:64-"
        // "f80:128-"
        // "n8:16:32:64-"
        // "S128"
        teprintln!("Self::align_l2():  {}", Self::align_l2());
        teprintln!("align_of::<ElemT>(): {}", std::mem::align_of::<ElemT>());
        teprintln!(
            "align_of_val(&self.a): {}",
            std::mem::align_of_val(&self_.a)
        );
        teprintln!("align_of::<Self>(): {}", std::mem::align_of::<Self>());

        teprintln!(
            "ElemT::BITS: {}, Self::align_n(): {}",
            ElemT::BITS,
            Self::align_n()
        );
        //assert!(ElemT::BITS <= (1 << (Self::align_l2() + 3)));
        //assert!(ElemT::BITS <= Self::align_n()*8);

        self_
    }

    pub fn is_zero(&self) -> bool {
        //? TODO transmute to larger size without swizzling
        self.a.iter().all(|elem| *elem == ElemT::ZERO)
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        let mut ix = 0;
        while ix != ARRAY_N {
            if self.a[ix] != other.a[ix] {
                return false;
            }
            ix += 1;
        }
        true
    }
}

impl<ElemT, const ARRAY_N: usize> PartialEq<Self> for FixedSizeArrayOfUnsigned<ElemT, ARRAY_N>
where
    ElemT: PrimitiveType<PrimitiveType = ElemT>,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl<ElemT, const ARRAY_N: usize> std::fmt::Debug for FixedSizeArrayOfUnsigned<ElemT, ARRAY_N>
where
    ElemT: PrimitiveType<PrimitiveType = ElemT>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "FixedUint<u{}, Self::align_n() = {}, Self::ALIGN_L2 = {}>",
            ElemT::BITS,
            Self::align_n(),
            Self::ALIGN_L2
        ))?;

        f.write_fmt(format_args!(
            "FixedUint<u{bits}, {ARRAY_N}> {{\n    a: [",
            bits = ElemT::BITS,
        ))?;

        for (ix, elem) in self.a.iter().enumerate() {
            if ix != 0 {
                f.write_str(",")?;
            }

            f.write_fmt(format_args!(
                "\n        {ix}: 0x{elem:0width$X}",
                width = ElemT::SIZE * 2
            ))?;
        }

        f.write_fmt(
            format_args!(
                "\n    ],\n    elem_order: {:?} '{ch}',\n    byte_order: {:?},\n    bit_order: {:?}\n}}",
                Self::elem_order(), Self::byte_order(), Self::bit_order(),
                ch = self.elem_order.to_debug_char()
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use static_assertions::*;
    use std::mem::{align_of, size_of};

    type FixedUint_128_4 = FixedSizeArrayOfUnsigned<
        u128, // ElemT
        4,    // ARRAY_N
    >;

    #[test]
    fn t00() {
        type UnitTestType = FixedUint_128_4;
        type ElemT = <UnitTestType as StorageOrganization>::T;

        const FIXEDUINT_128_4_ZERO: ElemT = ElemT::ZERO;

        // Tests std::fmt::Debug
        teprintln!("\n{FIXEDUINT_128_4_ZERO:?}");
    }

    #[test]
    fn t01() -> Result<()> {
        type UnitTestType = FixedUint_128_4;
        //type ElemT = <UnitTestType as StorageOrganization>::PrimitiveType;

        // Tests std::fmt::Debug
        teprintln!("\n{:?}", UnitTestType::new_fixed_bitpattern());

        Ok(())
    }

    type FixedUint_8_4 = FixedSizeArrayOfUnsigned<
        u8, // ElemT
        4,  // ARRAY_N
    >;

    #[test]
    fn t02() -> Result<()> {
        type UnitTestType = FixedUint_128_4;
        //type ElemT = <UnitTestType as StorageOrganization>::PrimitiveType;

        // Tests std::fmt::Debug
        teprintln!("\n{:?}", UnitTestType::new_fixed_bitpattern());

        teprintln!("\n========= ElemT = u8");
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 16>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 16>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 0>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u8, 0>::new_fixed_bitpattern());

        teprintln!("\n========= ElemT = u16");
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 3>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 3>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 0>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u16, 0>::new_fixed_bitpattern());

        teprintln!("\n========= ElemT = u32");
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u32, 3>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u32, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u32, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u32, 0>::new_fixed_bitpattern());

        teprintln!("\n========= ElemT = u64");
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u64, 3>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u64, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u64, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u64, 0>::new_fixed_bitpattern());

        teprintln!("\n========= ElemT = u128");
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u128, 3>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u128, 2>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u128, 1>::new_fixed_bitpattern());
        teprintln!("{:?}", FixedSizeArrayOfUnsigned::<u128, 0>::new_fixed_bitpattern());

        Ok(())
    }
}
