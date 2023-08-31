#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO
#![allow(unused_variables)] //? TODO
#![allow(unreachable_code)] //? TODO
#![allow(non_camel_case_types)] //? TODO

use crate::primitive_unsigned::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbsoluteEndian {
    Big = 0,
    Little = 1,
}

impl AbsoluteEndian {
    const ACTUAL_TARGET_ENDIAN: AbsoluteEndian = if cfg!(target_endian = "big") {
        AbsoluteEndian::Big
    } else {
        AbsoluteEndian::Little
    };

    const fn actual_target_endian() -> AbsoluteEndian {
        Self::ACTUAL_TARGET_ENDIAN
    }

    const fn from_usize(val: usize) -> AbsoluteEndian {
        match val {
            0 => AbsoluteEndian::Big,
            _ => AbsoluteEndian::Little,
        }
    }
    const fn to_usize(&self) -> usize {
        *self as usize
    }
    const fn is_big(&self) -> bool {
        matches!(self, &AbsoluteEndian::Big)
    }
    const fn is_little(&self) -> bool {
        matches!(self, &AbsoluteEndian::Little)
    }
    const fn other(&self) -> AbsoluteEndian {
        match self {
            AbsoluteEndian::Big => AbsoluteEndian::Little,
            AbsoluteEndian::Little => AbsoluteEndian::Big,
        }
    }
    const fn is_target(&self) -> bool {
        //*self == Self::actual_target_endian() // can't call in const fn
        self.to_usize() == Self::actual_target_endian().to_usize()
    }
    const fn is_swapped(&self) -> bool {
        !self.is_target()
    }

    const fn to_relative(&self) -> RelativeEndian {
        if self.is_target() {
            RelativeEndian::Target
        } else {
            RelativeEndian::Swapped
        }
    }
}

#[cfg(test)]
mod t_absoluteendian {
    use super::*;

    #[test]
    fn t_00() {
        assert_eq!(AbsoluteEndian::from_usize(0), AbsoluteEndian::Big);
        assert_eq!(AbsoluteEndian::from_usize(1), AbsoluteEndian::Little);
        assert_eq!(AbsoluteEndian::from_usize(2), AbsoluteEndian::Little);

        assert_eq!(AbsoluteEndian::Big.to_usize(), 0);
        assert_eq!(AbsoluteEndian::Little.to_usize(), 1);

        assert_eq!(AbsoluteEndian::Big.is_big(), true);
        assert_eq!(AbsoluteEndian::Little.is_big(), false);

        assert_eq!(AbsoluteEndian::Big.is_little(), false);
        assert_eq!(AbsoluteEndian::Little.is_little(), true);

        assert_eq!(AbsoluteEndian::Big.other(), AbsoluteEndian::Little);
        assert_eq!(AbsoluteEndian::Little.other(), AbsoluteEndian::Big);
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelativeEndian {
    Target = 0,
    Swapped = 1,
}
impl RelativeEndian {
    const fn from_usize(val: usize) -> RelativeEndian {
        debug_assert!(val < 2, "Invalid relative endian usize value");
        match val {
            0 => RelativeEndian::Target,
            _ => RelativeEndian::Swapped,
        }
    }
    const fn to_usize(&self) -> usize {
        *self as usize
    }
    const fn is_target(&self) -> bool {
        matches!(self, &RelativeEndian::Target)
    }
    const fn is_swapped(&self) -> bool {
        matches!(self, &RelativeEndian::Swapped)
    }
    const fn to_absolute(&self) -> AbsoluteEndian {
        #[cfg(target_endian = "little")]
        match self {
            RelativeEndian::Target => AbsoluteEndian::Little,
            RelativeEndian::Swapped => AbsoluteEndian::Big,
        }
        #[cfg(target_endian = "big")]
        match self {
            RelativeEndian::Target => AbsoluteEndian::Big,
            RelativeEndian::Swapped => AbsoluteEndian::Little,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t00() {}
}

/* /// A fixed-length array of RelativeEndian values.
struct RelativeEndianVec(u64);
impl RelativeEndianVec {

    /// Creates a relative endian vec.
    ///
    /// `ElemT` The element type `u8`, `u16`, ..., `u128`.
    ///
    /// `INDEX_BITS` The number of bits needed to index an element.
    ///
    const fn new_for<ElemT: PrimitiveUnsigned, const INDEX_BITS: u32>() -> RelativeEndianVec {
        //const ELEMT_BITS_L2: u32 = ElemT::BITS_L2 as u32;
        let intraelem_bit_index_bits = ElemT::BITS_L2 as u32;
        const INDEX_BITS: u32 = INDEX_BITS;

        //static_assertions::const_assert!(elemt_bits_l2 + INDEX_BITS <= u64::BITS);
        //const _:() = assert!(ElemT::BITS_L2 as u32 + INDEX_BITS <= u64::BITS);
        assert!(intraelem_bit_index_bits + INDEX_BITS <= u64::BITS);

        let intrabyte_bit_index_bits: u32 = 3; // 2^3 = 8 bits per byte
        let intraelem_byte_index_bits: u32 = ElemT::BITS_L2 as u32 - 3; // e.g. 2^2 = 4 bytes per u32
        let total_vec_bits: u32 = intrabyte_bit_index_bits + intraelem_byte_index_bits + INDEX_BITS;

        let elemt_name = ElemT::NAME;
        assert!(total_vec_bits <= u64::BITS,
            "RelativeEndianVec for ElemT={elemt_name}, INDEX_BITS={INDEX_BITS} doesn't fit in u64");

        let u: u64 = if total_vec_bits == u64::BITS { usize::MAX } else {
            (1 << INDEX_BITS) - 1
        };

        EndianVec(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t00() {
    }
} */
