#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

pub trait PrimitiveUnsigned:
    Clone
    + Copy
    + Sized
    + std::fmt::Debug
    + std::fmt::Display
    + std::fmt::Binary
    + std::fmt::LowerHex
    + std::fmt::UpperHex

    + std::ops::Shl<u8>
    + std::ops::Shl<u16>
    + std::ops::Shl<u32>
    + std::ops::Shl<u64>
    + std::ops::Shl<u128>
    + std::ops::Shl<usize, Output = Self>
    + std::ops::Shl<i8>
    + std::ops::Shl<i16>
    + std::ops::Shl<i32>
    + std::ops::Shl<i64>
    + std::ops::Shl<i128>
    + std::ops::Shl<isize>
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

    + std::ops::Shr<u8>
    + std::ops::Shr<u16>
    + std::ops::Shr<u32>
    + std::ops::Shr<u64>
    + std::ops::Shr<u128>
    + std::ops::Shr<usize>
    + std::ops::Shr<i8>
    + std::ops::Shr<i16>
    + std::ops::Shr<i32>
    + std::ops::Shr<i64>
    + std::ops::Shr<i128>
    + std::ops::Shr<isize>
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

    + std::ops::Add<Self>
    + for<'a> std::ops::Add<&'a Self>
    + std::ops::AddAssign<Self>
    + for<'a> std::ops::AddAssign<&'a Self>
    + std::ops::BitAnd<Self>
    + for<'a> std::ops::BitAnd<&'a Self>
    + std::ops::BitAndAssign<Self>
    + for<'a> std::ops::BitAndAssign<&'a Self>
    + std::ops::BitOr<Self>
    + for<'a> std::ops::BitOr<&'a Self>
    + std::ops::BitOrAssign<Self>
    + for<'a> std::ops::BitOrAssign<&'a Self>
    + std::ops::BitXor<Self>
    + for<'a> std::ops::BitXor<&'a Self>
    + std::ops::BitXorAssign<Self>
    + for<'a> std::ops::BitXorAssign<&'a Self>
    + std::ops::Div<Self>
    + for<'a> std::ops::Div<&'a Self>
    + std::ops::DivAssign<Self>
    + for<'a> std::ops::DivAssign<&'a Self>
    + std::ops::Mul<Self>
    + for<'a> std::ops::Mul<&'a Self>
    + std::ops::MulAssign<Self>
    + for<'a> std::ops::MulAssign<&'a Self>
    + std::cmp::PartialEq<Self>
    + std::cmp::Eq
    + std::cmp::PartialOrd<Self>
    + std::cmp::Ord
    + std::ops::Rem<Self>
    + for<'a> std::ops::Rem<&'a Self>
    + std::ops::RemAssign<Self>
    + for<'a> std::ops::RemAssign<&'a Self>
    + std::ops::Sub<Self>
    + for<'a> std::ops::Sub<&'a Self>
    + std::ops::SubAssign<Self>
    + for<'a> std::ops::SubAssign<&'a Self>
    + std::convert::From<bool>
    + std::convert::From<u8>
{
    const ALIGN: usize = std::mem::align_of::<Self>();
    const SIZE: usize = std::mem::size_of::<Self>();
    const BITS: usize = Self::SIZE * 8;
    const ZERO: Self;
    const ONE: Self;
}

pub trait PrimitiveUnsignedAtLeast16: PrimitiveUnsigned + std::convert::From<u16> {}

pub trait PrimitiveUnsignedAtLeast32: PrimitiveUnsignedAtLeast16 + std::convert::From<u32> {}

pub trait PrimitiveUnsignedAtLeast64: PrimitiveUnsignedAtLeast32 + std::convert::From<u64> {}

pub trait PrimitiveUnsignedAtLeast128:
    PrimitiveUnsignedAtLeast64 + std::convert::From<u128>
{
}

impl PrimitiveUnsigned for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl PrimitiveUnsigned for u16 {
    const ZERO: Self = 0u16;
    const ONE: Self = 1u16;
}
impl PrimitiveUnsignedAtLeast16 for u16 {}

impl PrimitiveUnsigned for u32 {
    const ZERO: Self = 0u32;
    const ONE: Self = 1u32;
}
impl PrimitiveUnsignedAtLeast16 for u32 {}
impl PrimitiveUnsignedAtLeast32 for u32 {}

impl PrimitiveUnsigned for u64 {
    const ZERO: Self = 0u64;
    const ONE: Self = 1u64;
}
impl PrimitiveUnsignedAtLeast16 for u64 {}
impl PrimitiveUnsignedAtLeast32 for u64 {}
impl PrimitiveUnsignedAtLeast64 for u64 {}

impl PrimitiveUnsigned for u128 {
    const ZERO: Self = 0u128;
    const ONE: Self = 1u128;
}
impl PrimitiveUnsignedAtLeast16 for u128 {}
impl PrimitiveUnsignedAtLeast32 for u128 {}
impl PrimitiveUnsignedAtLeast64 for u128 {}
impl PrimitiveUnsignedAtLeast128 for u128 {}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_primitiveunsigned<T: PrimitiveUnsigned>() {
        assert_eq!(T::SIZE, std::mem::size_of::<T>());
        assert_eq!(T::BITS, std::mem::size_of::<T>() * 8);
        assert_eq!(T::ZERO, 0.into());
        assert_eq!(T::ONE, 1.into());
    }

    #[test]
    fn it_works() {
        check_primitiveunsigned::<u8>();
        check_primitiveunsigned::<u16>();
        check_primitiveunsigned::<u32>();
        check_primitiveunsigned::<u64>();
        check_primitiveunsigned::<u128>();
    }
}
