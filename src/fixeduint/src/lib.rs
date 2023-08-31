#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO
#![allow(unused_variables)] //? TODO
#![allow(unreachable_code)] //? TODO
#![allow(non_camel_case_types)] //? TODO

mod bitvec_organization;
mod primitive_unsigned;

use static_assertions::*;
use std::mem::size_of;
use std::ops::Mul;
use typenum::{
    assert_type,
    consts::*,
    op,
    operator_aliases::Prod,
    type_operators::{Pow, Same},
    PowerOfTwo, Unsigned,
};

use crate::primitive_unsigned::PrimitiveUnsigned;

#[repr(transparent)]
struct FixedUint<
    ElemT: PrimitiveUnsigned,
    const ARRAY_N: usize,
    const ALIGN: usize,
    const ELEM_ENDIAN_BIG: bool,
>([ElemT; ARRAY_N]);

trait ArrayElemType {
    type Output: PrimitiveUnsigned;
}

impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > ArrayElemType for FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    type Output = ElemT;
}

impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    pub const ARRAY_N: usize = ARRAY_N;
    pub const ALIGN: usize = ALIGN;
    pub const ELEM_ENDIAN_BIG: bool = ELEM_ENDIAN_BIG;

    pub const ARRAY_ELEM_SIZE_OF: usize = ElemT::SIZE;
    pub const ARRAY_ELEM_BITS: u32 = ElemT::BITS;

    pub const ZERO: Self = Self([ElemT::ZERO; ARRAY_N]);

    pub fn is_zero(&self) -> bool {
        //? TODO transmute to larger size without swizzling
        self.0.iter().all(|elem| *elem == ElemT::ZERO)
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        let mut ix = 0;
        while ix != ARRAY_N {
            if self.0[ix] != other.0[ix] {
                return false;
            }
            ix += 1;
        }
        true
    }
}

impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > PartialEq<Self> for FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > From<[ElemT; ARRAY_N]> for FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    fn from(arr: [ElemT; ARRAY_N]) -> Self {
        Self(arr)
    }
}

impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > std::fmt::Debug for FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bl = if ELEM_ENDIAN_BIG { "B" } else { "L" };
        let bits = ElemT::BITS;
        f.write_fmt(format_args!("FixUint<u{bits}, {ARRAY_N}, {ALIGN}, {bl}>(["))?;
        for (ix, elem) in self.0.iter().enumerate() {
            if ix != 0 {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("0x{elem:0width$X}", width = ElemT::SIZE * 2))?;
        }
        f.write_str("])")
    }
}

/* fn is_one(&self) -> bool {
    if ARRAY_N == 0 {
        panic!("Overflow");
    } else {
        let ix_lse = if ELEM_ENDIAN_BIG { ARRAY_N - 1 } else { 0 };
        self.0.iter().enumerate().all(|(ix, elem)| {
            if ix != ix_lse {
                elem.is_zero()
            } else {
                elem.is_one()
            }
        })
    }
} */

#[cfg(test)]
impl<
        ElemT: PrimitiveUnsigned,
        const ARRAY_N: usize,
        const ALIGN: usize,
        const ELEM_ENDIAN_BIG: bool,
    > FixedUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>
{
    /// Creates a fixed bitpattern such that the upper nibble of every byte is the
    /// element index and the lower nibble is the byte index within the element.
    /// For testing.
    ///
    /// For example:
    ///
    /// FixUint<ElemT, ARRAY_N, ALIGN, ELEM_ENDIAN_BIG>([
    ///     3F3E3D3C3B3A39383736353433323130, 2F2E2D2C2B2A29282726252423222120,
    ///     1F1E1D1C1B1A19181716151413121110, 0F0E0D0C0B0A09080706050403020100
    /// ])
    ///
    pub fn make_fixed_bitpattern() -> Self {
        Self(std::array::from_fn(|elemix_usize| {
            let elemix_usize = if Self::ELEM_ENDIAN_BIG {
                Self::ARRAY_N.checked_sub(elemix_usize + 1).unwrap()
            } else {
                elemix_usize
            };
            let elemix_nibble = elemix_usize.min(15) as u8;

            let mut val_elemt = ElemT::ZERO;
            for byix_usize in 0..ElemT::SIZE {
                let shift_bits = byix_usize * 8;
                let by = (elemix_nibble << 4) | (byix_usize.min(15) as u8);
                val_elemt |= ElemT::from(by) << shift_bits;
            }

            val_elemt
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use static_assertions::*;
    use std::mem::{align_of, size_of};

    type FixedUint_128_4_16_B = FixedUint<
        u128,                   // ElemT
        4,                      // ARRAY_N
        { align_of::<u128>() }, // ALIGN
        true,                   // ELEM_ENDIAN_BIG
    >;

    #[test]
    fn t00() {
        const FIXEDUINT_128_4_T_ZERO: FixedUint_128_4_16_B = FixedUint_128_4_16_B::ZERO;

        // Tests std::fmt::Debug
        eprintln!("\n{FIXEDUINT_128_4_T_ZERO:?}");

        assert!(FIXEDUINT_128_4_T_ZERO == FixedUint_128_4_16_B::ZERO);
    }

    #[test]
    fn t01() -> Result<()> {
        type UnitTestType = FixedUint_128_4_16_B;
        //type ElemT = <UnitTestType as ArrayElemType>::Output;

        // Tests std::fmt::Debug
        eprintln!("\n{:?}", UnitTestType::make_fixed_bitpattern());

        Ok(())
    }

    type FixedUint_8_4_1_B = FixedUint<
        u8,                   // ElemT
        4,                    // ARRAY_N
        { align_of::<u8>() }, // ALIGN
        true,                 // ELEM_ENDIAN_BIG
    >;

    #[test]
    fn t02() -> Result<()> {
        type UnitTestType = FixedUint_128_4_16_B;
        //type ElemT = <UnitTestType as ArrayElemType>::Output;

        // Tests std::fmt::Debug
        eprintln!("\n");
        eprintln!("{:?}", FixedUint_8_4_1_B::make_fixed_bitpattern());

        eprintln!("\n========= ElemT = u8");
        eprintln!(
            "{:?}",
            FixedUint::<u8, 16, 1, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u8, 16, 1, false>::make_fixed_bitpattern()
        );
        eprintln!("{:?}", FixedUint::<u8, 2, 1, true>::make_fixed_bitpattern());
        eprintln!(
            "{:?}",
            FixedUint::<u8, 2, 1, false>::make_fixed_bitpattern()
        );
        eprintln!("{:?}", FixedUint::<u8, 1, 1, true>::make_fixed_bitpattern());
        eprintln!(
            "{:?}",
            FixedUint::<u8, 1, 1, false>::make_fixed_bitpattern()
        );
        eprintln!("{:?}", FixedUint::<u8, 0, 1, true>::make_fixed_bitpattern());
        eprintln!(
            "{:?}",
            FixedUint::<u8, 0, 1, false>::make_fixed_bitpattern()
        );

        eprintln!("\n========= ElemT = u16");
        eprintln!(
            "{:?}",
            FixedUint::<u16, 3, 2, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 3, 2, false>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 2, 2, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 2, 2, false>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 1, 2, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 1, 2, false>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 0, 2, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u16, 0, 2, false>::make_fixed_bitpattern()
        );

        eprintln!("\n========= ElemT = u32");
        eprintln!(
            "{:?}",
            FixedUint::<u32, 3, 4, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u32, 2, 4, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u32, 1, 4, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u32, 0, 4, true>::make_fixed_bitpattern()
        );

        eprintln!("\n========= ElemT = u64");
        eprintln!(
            "{:?}",
            FixedUint::<u64, 3, 8, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u64, 2, 8, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u64, 1, 8, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u64, 0, 8, true>::make_fixed_bitpattern()
        );

        eprintln!("\n========= ElemT = u128");
        eprintln!(
            "{:?}",
            FixedUint::<u128, 3, 16, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u128, 2, 16, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u128, 1, 16, true>::make_fixed_bitpattern()
        );
        eprintln!(
            "{:?}",
            FixedUint::<u128, 0, 16, true>::make_fixed_bitpattern()
        );

        Ok(())
    }
}
