#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO
#![allow(unused_variables)] //? TODO
#![allow(unreachable_code)] //? TODO
#![allow(non_camel_case_types)] //? TODO

//! ### Concepts and terms
//!
//! #### Sequence
//!
//! A 'sequence' is that thing that you can 'iterate': an operation that yields one 'element' after
//! another in a well-defined order until either it runs out or you decide to stop.
//!
//! #### Sequence order
//!
//! Rust's [`std::cmp::PartialOrd`] and [`std::cmp::Ord`] traits describe whether two objects
//! can be compared for ordering, based on their types. But elements of a sequence are generally
//! not required to implement these traits. Instead, sequences have an intrinsic order defined by
//! which element is yielded [first before the rest](std::iter::Iterator::next()) when iterated.
//!
//! This leads to [a natural correspondence with](std::iter::Iterator::enumerate()) integer ordinal
//! 'index' values beginning with 0. This index type is often `usize`, which *does* implement
//! `std::cmp::Ord`.
//!
//! #### Ordered sequences
//!
//! Some container types specialize in ensuring that their natural order of iteration matches
//! (or at least does not conflict with) the elements' ordering. I.e., no subsequent object will
//! compare [`std::cmp::Ordering::Less`] than a previous object.
//! Such containers require their element type to implement [`std::cmp::Ord`] and the container
//! may not even implement [`std::ops::Index`]. An example is [`std::collections::btree_set::BTreeSet`].
//!
//! This type of ordered sequence is *not* what is being discussed here.
//!
//! #### Indexing
//!
//! Array-based container types can access elements in O(1) time using an index value of type
//! `usize`. Rust provides [special syntax](std::ops::Index) to make this convenient. Ensuring
//! that index values stay within the sequence's defined range is a *huge* consideration.
//!
//! #### Endianness
//!
//! Sometimes it's necessary to break up a larger thing into multiple smaller things. If you
//! hope to ever correctly interpret it as the larger thing again, the resulting collection
//! of smaller things probably needs to preserve sequence order.
//!
//! For example, humans commonly represent integers as a sequence of digits using the order to
//! indicate the significance of each digit.
//!
//! There are two equally-obvious choices here: the significance the digits can increase
//! progressively through the sequence, or it can decrease.
//!
//! * When the least-significant digit comes first and monotonically increases through the sequence,
//! it's referred to as 'little-endian'.
//!
//! * When the most-significant digit comes first and monotonically decreases through the sequence,
//! it's referred to as 'big-endian'.
//!
//! Each has advantages in different situations.
//!
//! #### Other ways to describe the order of elements within a collection
//!
//! Not all sequences of smaller things are numbers. For example, the bytes of an "ASCII" text
//! string or file definitely represent characters iterated in a specific order, but they do not
//! have the concept of most- or least-significant of the within the text. We will simply call
//! this 'sequential' or 'forward' ordering.
//!
//! Sometimes we want to traverse, or refer to, a sequence in 'reverse' order.
//!
//! And not all collections of things even need a sequence order. A box of crayons leaves the
//! factory with a particular sequence of colors, but that ordering is not necessisarily
//! meaningful to its users and is not expected to be maintained over time.
//! However, this distinction between 'not ordered' and 'sequential' is rarely relevant.
//! (Note that even in this example the user may still have preferences on the stored order or at
//! least the expectation that it will be preserved across operations. "I think I put those colors
//! I was using last time in this general area". Good software should enable that even if it
//! wasn't envisioned in advance by the developers.)
//!
//! A sequence's ordering may even be signifiant for what we *can't* say about it. For example,
//! elaborate "shuffling" rituals have been standardized for ensuring, in a way verifiable by
//! all the participants, that *no one* has *any* information about the ordering of cards within
//! a deck. This is outside the scope of this crate.
//!
//! #### Summary
//!
//! |                         | little-endian |   big-endian    |  sequential   |
//! | ----------------------- | ------------- | --------------- | ------------- |
//! |         example         |   x86 WORDs   | decimal numbers | chars in text |
//! | least significant digit |     first     |     last        |               |
//! | most significant digit  |     last      |     first       |               |
//!

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SequenceOrder {
    /// The storage units or array elements are in the sequence natural for what the array
    /// as a whole represents.
    Forward = 0_u8,

    /// The storage units or array elements are in reversed order.
    Reverse = 1_u8,
}

impl SequenceOrder {
    pub fn to_debug_char(self) -> char {
        match self {
            SequenceOrder::Forward => 'F',
            SequenceOrder::Reverse => 'R',
        }
    }
}

/// The order of significance of the smaller things which make up a larger thing.
///
/// The ByteOrder type exists for specifically describing the order of bytes within
/// a storage unit.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endian {
    /// The least-significant smaller thing comes first, and the most-significant last.
    Little = 0_u8,

    /// The most-significant smaller thing comes first, and the least-significant last.
    /// Sometimes referred to as 'network byte order'.
    Big = 1_u8,
}

impl Endian {
    /// The endian of the "target" system architecture for which the code
    /// is being compiled.
    pub const fn target() -> Endian {
        if cfg!(target_endian = "little") {
            Endian::Little
        } else {
            Endian::Big
        }
    }
    pub fn to_debug_char(self) -> char {
        match self {
            Endian::Little => 'L',
            Endian::Big => 'B',
        }
    }
}

/// Used to describe the order of elements within a collection.
///
/// It may make sense to describe a collection as `Forward`, `Backward`, `Little`, or `Big`.
/// It's not clear that it would make sense to describe a collection as `Native` or `Opposite`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SequenceOrEndian {
    Sequence(SequenceOrder),
    Endian(Endian),
}

impl SequenceOrEndian {
    pub fn to_debug_char(self) -> char {
        match self {
            SequenceOrEndian::Sequence(so) => so.to_debug_char(),
            SequenceOrEndian::Endian(en) => en.to_debug_char(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelativeEndian {
    /// Matches the "target" system, i.e., the architecture for which the code
    /// is being compiled. E.g., `Little` on x86.
    Native = 0_u8,

    /// The order of bytes within a machine word are reversed relative to the "target" system.
    Opposite = 1_u8,
}

impl RelativeEndian {
    pub fn to_debug_char(self) -> char {
        match self {
            RelativeEndian::Native => 'N',
            RelativeEndian::Opposite => 'O',
        }
    }
}

/// The order of bytes within a storage unit or array element, and how that order
/// relates to the target architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteOrder {
    pub absolute_endian: Endian,
    pub relative_endian: RelativeEndian,
}

impl ByteOrder {
    pub fn to_debug_char(self) -> char {
        self.absolute_endian.to_debug_char() //? relative_endian?
    }
}

/// The order of bits within a byte. This seems fundamentally `[SequenceOrder::Forward]`
/// or `[RelativeEndian::Native]`, by definition, but perhaps we will need to more options
/// at some point in the future.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitOrder {
    Forward = SequenceOrder::Forward as u8,
}

impl BitOrder {
    pub fn to_debug_char(self) -> char {
        match self {
            BitOrder::Forward => 'F',
        }
    }
}

// pub struct HierarchicalOrder {
//     /// Order of the elements of the allocation array.
//     sequence_order: SequenceOrder,
//     byte_endian: (ByteOrder, RelativeByteEndian),
//     bit_order: BitOrder,
// }

/*
/// Absolute representation of endianness: Little, Big, Forward, or Reverse.
///
/// The target host's endianness is always represented by the `u8` value of 0, so it's probably
/// not a good idea to serialize this value to storage or network.
///
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbsEndian {
    /// The least-significant byte comes first and the most-significant last.
    Little = Self::LITTLE_U8,

    /// The most-significant byte comes first and the least-significant last.
    Big = Self::BIG_U8,

    /// The elements are in the sequence designated as 'forward'.
    Forward = Self::FORWARD_U8,

    /// The element sequence is reverse.
    Reverse = Self::REVERSE_U8,
}

impl AbsEndian {
    #[doc(hidden)]
    pub const TARGET_U8: u8 = 0;

    #[doc(hidden)]
    pub const OPPOSITE_U8: u8 = 1;

    #[doc(hidden)]
    pub const LITTLE_U8: u8 = cfg!(target_endian = "big") as u8; // 0 on little-endian, 1 on big.

    #[doc(hidden)]
    pub const BIG_U8: u8 = Self::LITTLE_U8 ^ 1; // 1 on little-endian, 0 on big.

    #[doc(hidden)]
    pub const FORWARD_U8: u8 = 2;

    #[doc(hidden)]
    pub const REVERSE_U8: u8 = 3;

    /// The maximum `u8` discriminant value.
    pub const MAX_U8: u8 = Self::REVERSE_U8;

    /// The endian of the target host.
    pub const TARGET: AbsEndian = AbsEndian::from_u8_panics(Self::TARGET_U8);

    /// The opposite endian of the target host.
    pub const OPPOSITE: AbsEndian = AbsEndian::from_u8_panics(Self::OPPOSITE_U8);

    /// Returns true if one is `Little` and the other is `Big`, in either order.
    pub const fn is_swapped_with(self, other: AbsEndian) -> bool {
        matches!((self, other), (AbsEndian::Little, AbsEndian::Big) | (AbsEndian::Big, AbsEndian::Little))
    }

    /// Converts a `u8` value to an `AbsEndian` value. Panics if the value is greater than `[MAX_U8]`.
    pub const fn from_u8_panics(v: u8) -> AbsEndian {
        match v {
            Self::LITTLE_U8 => AbsEndian::Little,
            Self::BIG_U8 => AbsEndian::Big,
            Self::FORWARD_U8 => AbsEndian::Forward,
            Self::REVERSE_U8 => AbsEndian::Reverse,
            _ => panic!("Invalid AbsEndian value"),
        }
    }

    /// Attempts to convert a `u8` value to an `AbsEndian` value.
    /// Returns `None` if the value is greater than `[MAX_U8]`.
    pub const fn try_from_u8(val: u8) -> Option<AbsEndian> {
        if val <= AbsEndian::MAX_U8 {
            Some(Self::from_u8_panics(val))
        } else {
            None
        }
    }

    /// Returns `true` if `self` is `[Big]`.
    const fn is_big(&self) -> bool {
        matches!(self, &AbsEndian::Big)
    }

    /// Returns `true` if `self` is `[Little]`.
    const fn is_little(&self) -> bool {
        matches!(self, &AbsEndian::Little)
    }

    /// Returns `true` if `self` is `[Forward]`.
    const fn is_forward(&self) -> bool {
        matches!(self, &AbsEndian::Forward)
    }

    /// Returns `true` if `self` is `[Reverse]`.
    const fn is_reverse(&self) -> bool {
        matches!(self, &AbsEndian::Reverse)
    }
}

#[cfg(test)]
mod t_absendian {
    use super::*;

    #[test]
    fn t() {
        assert_eq!(AbsEndian::TARGET as u8, 0);
        assert_eq!(AbsEndian::OPPOSITE as u8, 1);
        assert_eq!(AbsEndian::Forward as u8, 2);
        assert_eq!(AbsEndian::Reverse as u8, 3);

        assert_eq!(
            AbsEndian::from_u8_panics(AbsEndian::TARGET_U8),
            if cfg!(target_endian = "little") {
                AbsEndian::Little
            } else {
                AbsEndian::Big
            }
        );
        assert_eq!(
            AbsEndian::from_u8_panics(AbsEndian::OPPOSITE_U8),
            if cfg!(target_endian = "little") {
                AbsEndian::Big
            } else {
                AbsEndian::Little
            }
        );
        assert_eq!(
            AbsEndian::from_u8_panics(AbsEndian::FORWARD_U8),
            AbsEndian::Forward
        );
        assert_eq!(
            AbsEndian::from_u8_panics(AbsEndian::REVERSE_U8),
            AbsEndian::Reverse
        );


        assert!(AbsEndian::Little.is_little());
        assert!(AbsEndian::Big.is_big());
        assert!(AbsEndian::Forward.is_forward());
        assert!(AbsEndian::Reverse.is_reverse());

        assert!(!AbsEndian::Little.is_swapped_with(AbsEndian::Little));
        assert!(AbsEndian::Little.is_swapped_with(AbsEndian::Big));
        assert!(!AbsEndian::Little.is_swapped_with(AbsEndian::Forward));
        assert!(!AbsEndian::Little.is_swapped_with(AbsEndian::Reverse));

        assert!(!AbsEndian::Big.is_swapped_with(AbsEndian::Big));
        assert!(!AbsEndian::Big.is_swapped_with(AbsEndian::Forward));
        assert!(!AbsEndian::Big.is_swapped_with(AbsEndian::Reverse));

        assert!(!AbsEndian::Forward.is_swapped_with(AbsEndian::Forward));
        assert!(!AbsEndian::Forward.is_swapped_with(AbsEndian::Reverse));

        assert!(!AbsEndian::Forward.is_swapped_with(AbsEndian::Reverse));

        assert!(AbsEndian::TARGET.is_swapped_with(AbsEndian::OPPOSITE));
    }
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
*/
