#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

pub trait PrimitiveType:
    Sized
    + Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + std::fmt::Binary
    + std::fmt::LowerHex
    + std::fmt::UpperHex
    + std::cmp::PartialEq<Self>
    + std::cmp::Eq
    + std::cmp::PartialOrd<Self>
    + std::cmp::Ord
    + std::ops::Shl<u8, Output = Self>
    + std::ops::Shl<u16, Output = Self>
    + std::ops::Shl<u32, Output = Self>
    + std::ops::Shl<u64, Output = Self>
    + std::ops::Shl<u128, Output = Self>
    + std::ops::Shl<usize, Output = Self>
    + std::ops::Shl<i8, Output = Self>
    + std::ops::Shl<i16, Output = Self>
    + std::ops::Shl<i32, Output = Self>
    + std::ops::Shl<i64, Output = Self>
    + std::ops::Shl<i128, Output = Self>
    + std::ops::Shl<isize, Output = Self>
    + std::ops::ShlAssign<u8>
    + std::ops::ShlAssign<u16>
    + std::ops::ShlAssign<u32>
    + std::ops::ShlAssign<u64>
    + std::ops::ShlAssign<u128>
    + std::ops::ShlAssign<usize>
    + std::ops::ShlAssign<i8>
    + std::ops::ShlAssign<i16>
    + std::ops::ShlAssign<i32>
    + std::ops::ShlAssign<i64>
    + std::ops::ShlAssign<i128>
    + std::ops::ShlAssign<isize>
    + std::ops::Shr<u8, Output = Self>
    + std::ops::Shr<u16, Output = Self>
    + std::ops::Shr<u32, Output = Self>
    + std::ops::Shr<u64, Output = Self>
    + std::ops::Shr<u128, Output = Self>
    + std::ops::Shr<usize, Output = Self>
    + std::ops::Shr<i8, Output = Self>
    + std::ops::Shr<i16, Output = Self>
    + std::ops::Shr<i32, Output = Self>
    + std::ops::Shr<i64, Output = Self>
    + std::ops::Shr<i128, Output = Self>
    + std::ops::Shr<isize, Output = Self>
    + std::ops::ShrAssign<u8>
    + std::ops::ShrAssign<u16>
    + std::ops::ShrAssign<u32>
    + std::ops::ShrAssign<u64>
    + std::ops::ShrAssign<u128>
    + std::ops::ShrAssign<usize>
    + std::ops::ShrAssign<i8>
    + std::ops::ShrAssign<i16>
    + std::ops::ShrAssign<i32>
    + std::ops::ShrAssign<i64>
    + std::ops::ShrAssign<i128>
    + std::ops::ShrAssign<isize>
    + std::ops::Add<Self, Output = Self>
    + for<'a> std::ops::Add<&'a Self, Output = Self>
    + std::ops::AddAssign<Self>
    + for<'a> std::ops::AddAssign<&'a Self>
    + std::ops::BitAnd<Self, Output = Self>
    + for<'a> std::ops::BitAnd<&'a Self, Output = Self>
    + std::ops::BitAndAssign<Self>
    + for<'a> std::ops::BitAndAssign<&'a Self>
    + std::ops::BitOr<Self, Output = Self>
    + for<'a> std::ops::BitOr<&'a Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + for<'a> std::ops::BitOrAssign<&'a Self>
    + std::ops::BitXor<Self, Output = Self>
    + for<'a> std::ops::BitXor<&'a Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + for<'a> std::ops::BitXorAssign<&'a Self>
    + std::ops::Div<Self, Output = Self>
    + for<'a> std::ops::Div<&'a Self, Output = Self>
    + std::ops::DivAssign<Self>
    + for<'a> std::ops::DivAssign<&'a Self>
    + std::ops::Mul<Self, Output = Self>
    + for<'a> std::ops::Mul<&'a Self, Output = Self>
    + std::ops::MulAssign<Self>
    + for<'a> std::ops::MulAssign<&'a Self>
    + std::ops::Rem<Self, Output = Self>
    + for<'a> std::ops::Rem<&'a Self, Output = Self>
    + std::ops::RemAssign<Self>
    + for<'a> std::ops::RemAssign<&'a Self>
    + std::ops::Sub<Self, Output = Self>
    + for<'a> std::ops::Sub<&'a Self, Output = Self>
    + std::ops::SubAssign<Self>
    + for<'a> std::ops::SubAssign<&'a Self>
    + std::convert::From<bool>
    + std::convert::From<u8>
    + std::convert::Into<u128>
{
    type PrimitiveType;

    const NAME: &'static str;
    const ALIGN: usize = std::mem::align_of::<Self>();
    const SIZE: usize = std::mem::size_of::<Self>();
    const BITS_L2: u8;
    const BITS: u32 = 1u32 << Self::BITS_L2;
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
}

//---------- `AtMost` types

pub trait PrimitiveUnsignedAtMost128: PrimitiveType {}

pub trait PrimitiveUnsignedAtMost64: PrimitiveUnsignedAtMost128 + std::convert::Into<u64> {}

pub trait PrimitiveUnsignedAtMost32: PrimitiveUnsignedAtMost64 + std::convert::Into<u32> {}

pub trait PrimitiveUnsignedAtMost16: PrimitiveUnsignedAtMost32 + std::convert::Into<u16> {}

pub trait PrimitiveUnsignedAtMost8: PrimitiveUnsignedAtMost16 + std::convert::Into<u8> {}

//---------- `AtLeast` types

pub trait PrimitiveUnsignedAtLeast8: PrimitiveType {}

pub trait PrimitiveUnsignedAtLeast16: PrimitiveUnsignedAtLeast8 + std::convert::From<u16> {}

pub trait PrimitiveUnsignedAtLeast32: PrimitiveUnsignedAtLeast16 + std::convert::From<u32> {}

pub trait PrimitiveUnsignedAtLeast64: PrimitiveUnsignedAtLeast32 + std::convert::From<u64> {}

pub trait PrimitiveUnsignedAtLeast128:
    PrimitiveUnsignedAtLeast64 + std::convert::From<u128>
{
}

//------ impls on concrete types

//------ u8

impl PrimitiveType for u8 {
    type PrimitiveType = u8;
    const NAME: &'static str = "u8";
    const BITS_L2: u8 = 3;
    const ZERO: <Self as PrimitiveType>::PrimitiveType = 0;
    const ONE: Self = 1;
    const MAX: Self = u8::MAX;
}

impl PrimitiveUnsignedAtMost8 for u8 {}
impl PrimitiveUnsignedAtMost16 for u8 {}
impl PrimitiveUnsignedAtMost32 for u8 {}
impl PrimitiveUnsignedAtMost64 for u8 {}
impl PrimitiveUnsignedAtMost128 for u8 {}

impl PrimitiveUnsignedAtLeast8 for u8 {}

//------ u16

impl PrimitiveType for u16 {
    type PrimitiveType = u16;
    const NAME: &'static str = "u16";
    const BITS_L2: u8 = 4;
    const ZERO: <Self as PrimitiveType>::PrimitiveType = 0;
    //const ZERO: Self = 0u16;
    const ONE: Self = 1u16;
    const MAX: Self = u16::MAX;
}

impl PrimitiveUnsignedAtMost16 for u16 {}
impl PrimitiveUnsignedAtMost32 for u16 {}
impl PrimitiveUnsignedAtMost64 for u16 {}
impl PrimitiveUnsignedAtMost128 for u16 {}

impl PrimitiveUnsignedAtLeast8 for u16 {}
impl PrimitiveUnsignedAtLeast16 for u16 {}

//------ u32

impl PrimitiveType for u32 {
    type PrimitiveType = u32;
    const NAME: &'static str = "u32";
    const BITS_L2: u8 = 5;
    const ZERO: <Self as PrimitiveType>::PrimitiveType = 0;
    //const ZERO: Self = 0u32;
    const ONE: Self = 1u32;
    const MAX: Self = u32::MAX;
}

impl PrimitiveUnsignedAtMost32 for u32 {}
impl PrimitiveUnsignedAtMost64 for u32 {}
impl PrimitiveUnsignedAtMost128 for u32 {}

impl PrimitiveUnsignedAtLeast8 for u32 {}
impl PrimitiveUnsignedAtLeast16 for u32 {}
impl PrimitiveUnsignedAtLeast32 for u32 {}

//------ u64

impl PrimitiveType for u64 {
    type PrimitiveType = u64;
    const NAME: &'static str = "u64";
    const BITS_L2: u8 = 6;
    const ZERO: <Self as PrimitiveType>::PrimitiveType = 0;
    //const ZERO: Self = 0u64;
    const ONE: Self = 1u64;
    const MAX: Self = u64::MAX;
}

impl PrimitiveUnsignedAtMost64 for u64 {}
impl PrimitiveUnsignedAtMost128 for u64 {}

impl PrimitiveUnsignedAtLeast8 for u64 {}
impl PrimitiveUnsignedAtLeast16 for u64 {}
impl PrimitiveUnsignedAtLeast32 for u64 {}
impl PrimitiveUnsignedAtLeast64 for u64 {}

//------ u128

impl PrimitiveType for u128 {
    type PrimitiveType = u128;
    const NAME: &'static str = "u128";
    const BITS_L2: u8 = 7;
    const ZERO: <Self as PrimitiveType>::PrimitiveType = 0;
    const ONE: Self = 1u128;
    const MAX: Self = u128::MAX;
}

impl PrimitiveUnsignedAtMost128 for u128 {}

impl PrimitiveUnsignedAtLeast8 for u128 {}
impl PrimitiveUnsignedAtLeast16 for u128 {}
impl PrimitiveUnsignedAtLeast32 for u128 {}
impl PrimitiveUnsignedAtLeast64 for u128 {}
impl PrimitiveUnsignedAtLeast128 for u128 {}

//------

pub const PRIMITIVEUNSIGNED_BITS_L2_MIN: u8 = 3;
pub const PRIMITIVEUNSIGNED_BITS_L2_MAX: u8 = 7;
pub const PRIMITIVEUNSIGNED_BITS_L2_VALID_RANGE: std::ops::Range<u8> =
    PRIMITIVEUNSIGNED_BITS_L2_MIN..(PRIMITIVEUNSIGNED_BITS_L2_MAX + 1);

pub const PRIMITIVEUNSIGNED_BITS_MAX: u32 = 1 << PRIMITIVEUNSIGNED_BITS_L2_MAX;

/* trait WiderOf { type Type; }
impl WiderOf for (u8, u8  ) { type Type = u8; }
impl WiderOf for (u8, u16 ) { type Type = u16; }
impl WiderOf for (u8, u32 ) { type Type = u32; }
impl WiderOf for (u8, u64 ) { type Type = u64; }
impl WiderOf for (u8, u128) { type Type = u128; }
impl WiderOf for (u16, u8  ) { type Type = u16; }
impl WiderOf for (u16, u16 ) { type Type = u16; }
impl WiderOf for (u16, u32 ) { type Type = u32; }
impl WiderOf for (u16, u64 ) { type Type = u64; }
impl WiderOf for (u16, u128) { type Type = u128; }
impl WiderOf for (u32, u8  ) { type Type = u32; }
impl WiderOf for (u32, u16 ) { type Type = u32; }
impl WiderOf for (u32, u32 ) { type Type = u32; }
impl WiderOf for (u32, u64 ) { type Type = u64; }
impl WiderOf for (u32, u128) { type Type = u128; }
impl WiderOf for (u64, u8  ) { type Type = u64; }
impl WiderOf for (u64, u16 ) { type Type = u64; }
impl WiderOf for (u64, u32 ) { type Type = u64; }
impl WiderOf for (u64, u64 ) { type Type = u64; }
impl WiderOf for (u64, u128) { type Type = u128; }
impl WiderOf for (u128, u8  ) { type Type = u128; }
impl WiderOf for (u128, u16 ) { type Type = u128; }
impl WiderOf for (u128, u32 ) { type Type = u128; }
impl WiderOf for (u128, u64 ) { type Type = u128; }
impl WiderOf for (u128, u128) { type Type = u128; } */

/// Expands the statements for each of `u8` through `u128`, defining the type alias
/// $UPT to each in turn.
#[macro_export]
macro_rules! for_each_fixed_width_unsigned_primitive_type {
    ($UPT:ident => $( $s:stmt );*) => {{
        { type $UPT = u8; $( $s );* }
        { type $UPT = u16; $( $s );* }
        { type $UPT = u32; $( $s );* }
        { type $UPT = u64; $( $s );* }
        { type $UPT = u128; $( $s );* }
    }};
}

#[inline(always)]
#[must_use]
fn pow2_minus_1_saturating<T: PrimitiveType>(n: u32) -> T {
    if n < T::BITS {
        (T::ONE << n) - T::ONE
    } else {
        T::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t00() {
        fn check_primitiveunsigned<T>()
        where
            T: PrimitiveType<PrimitiveType = T>,
            T: std::convert::Into<u128>,
        {
            let size_of_t = std::mem::size_of::<T>() as u64;
            let t_zero = T::ZERO;

            assert_eq!(T::SIZE as u64, size_of_t);
            assert_eq!(1 << T::BITS_L2, T::BITS as u64);
            assert_eq!(T::BITS as u64, size_of_t * 8);
            assert_eq!(T::ZERO, T::from(false));
            assert_eq!(T::ONE, T::from(true));

            assert!(PRIMITIVEUNSIGNED_BITS_L2_VALID_RANGE.contains(&T::BITS_L2));

            let check_primitiveunsigned_shift_count = |n: u32| {
                let t = pow2_minus_1_saturating::<T>(n);
                let t128 = Into::<u128>::into(t);
                eprintln!("t128={:b}", t128);
                let expected_ones: u32 = n.min(T::BITS);
                assert_eq!(t128.count_ones(), expected_ones);
                if 0 < n {
                    assert_eq!(t & T::ONE, T::ONE);
                }
            };

            check_primitiveunsigned_shift_count(0);
            check_primitiveunsigned_shift_count(1);
            check_primitiveunsigned_shift_count(2);
            check_primitiveunsigned_shift_count(T::BITS - 2);
            check_primitiveunsigned_shift_count(T::BITS - 1);
            check_primitiveunsigned_shift_count(T::BITS);
            check_primitiveunsigned_shift_count(T::BITS + 1);
        }

        check_primitiveunsigned::<u8>();
        check_primitiveunsigned::<u16>();
        check_primitiveunsigned::<u32>();
        check_primitiveunsigned::<u64>();
        check_primitiveunsigned::<u128>();
        assert_eq!(PRIMITIVEUNSIGNED_BITS_MAX, 128);
    }
}
