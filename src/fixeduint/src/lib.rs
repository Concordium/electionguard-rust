#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]
#![allow(dead_code)] //? TODO
#![allow(unused_imports)] //? TODO
#![allow(unused_variables)] //? TODO
#![allow(unreachable_code)] //? TODO
#![allow(non_camel_case_types)] //? TODO

/*!
# `fixeduint`

`fixeduint` is a collection of types and functions for representing and transforming
fixed-size buffers of Rust-native unsigned integers.

#### The modules of this crate

* [`endian`](crate::endian)
* [`bitvec_organization`](crate::bitvec_organization)
* [`fixeduint`](crate::fixeduint)
* [`primitive_unsigned`](crate::primitive_unsigned) Trait for describing `u8`, `u16`, `u32`,
`u64`, and `u128`, specifically.
* [`with_t_upt`](crate::with_t_upt) Macros for working with generic types `T` when `T` is
`PrimitiveUnsigned`.

#### Why do we need this?

Isn't a `slice<uXX>` (pointer and length, basically) enough?

Well, let's consider the simple case of a buffer allocated as an array of a fixed-size
unsigned primitive type. This is defined by the parameters:

* `[T; N]`, the allocation array type.<br/>

* `ALIGN_L2` The allocation unit type's alignment will be a power of two, so we
represent it as the log<sub>2</sub> of the alignment value. E.g. the `ALIGN_L2` of `u64` is 3.
 
* `T`, the allocation unit type, one of `u8`, `u16`, `u32`, `u64`, or `u128`. (Stay away
from `usize` for this.) These types have the nice property that their alignment and size are
the same. (See [`primitive_unsigned`](crate::primitive_unsigned) for traits which apply to
specifically these types.)

* `N`, the number of allocation array units. Total size (`size_of::<[T; N]>()`) should be
exactly `(1 << ALIGN_L2)*N`.

That desribes the storage allocation. If that representation meets all our needs, great,
we shouldn't even have to worry about silly details like "endianness" unless it leaves
our code via file or network.

But sometimes we will need to access this buffer using a different representation for
efficiency or convenience. For example, we may wish to operate on the buffer logically as a
sequence of bits, bytes, hex digits, or some CPU-specific register-size values.

This obligates us to track additional information:

* `[U; M]`, the access array type, having size and alignment values not greater than the
allocation array type. It MUST NOT be undefined to use `std::mem::transmute` between this and
the allocation array type. The size of `[U; M]` MUST NOT be greater than that of `[T; N]`.
(It could usefully be less, but we're trying to keep this example simple.) This implies that
the size and alignment of `U` MUST NOT be greater than that of `T`, so `T`
should be chosen wisely in advance of need.

* `U`, the access array element type. We will again stick with one of `u8`, `u16`, `u32`,
`u64`, or `u128` for simplicity. Again, `U` can not have a larger alignment requirement
than `T` and will often be smaller.

* `M`, the number of access array elements.

#### Now for the fun part

Whether you're referring to your data as an array of `T` or `U`, you probably care about
their order. This is particularly true when you're representing a larger thing as a sequence
of smaller things, such as a text document as `u8`s. There are commonly multiple levels of
representation involved. Consider the 4096 bits of a large integer, packed inside `u8` bytes,
packed inside aligned `u128` values to enable a CPU's wide arithmetic instructions.

Welcome to Endian-ville. Population: you.

(Consider reading [`endian`](crate::endian) for more about endianness and the ways it can
be described.)

We'll need a syntax to talk about how data is arranged in the allocation array, and how we might
request access to it in alternate forms.

Endianness can apply differently at multiple levels. For the purposes of this crate there are
typically three:

* The data within the overall allocation array, i.e., the order of the allocation elements.
For example, a large integer implementation may decide to store the most-significant data
first ("overall big-endian").

* The smaller units within an allocation element, e.g. `u8`s within a `u64`.
The example "overall big-endian" large integer implementation may still prefer to follow the
target's native endian to order the bytes within each allocation array element, to be compatible
with the target CPU's arithmetic instructions.

* The bits within a byte. Because these are accessed using shift and mask operations rather than
array indexing, this level is mostly outside the scope of this crate. They could be considered
'forward sequential' because they have a well-defined order, or 'little endian' because the
smallest shift count generally accesses the least significant bit. (But like other numbers
we usually write them in big-endian order.)

#### Example

Consider the example of using a `[u64; 13]` to represent a single 832-bit integer, on a
little-endian target with the least significant `u64` first:

* Bits 0..=2 of an index value refer to bits within a byte. Because they must be accessed
with value arithmetic, we can consider them to be `Little` endian.

* Bits 3..=5 index the `u8` within the `u64` allocation unit. These are `Target` endian,
which in this case is `Little`.
(This is `ALIGN_L2` number of bits)

* Bits 6..=9 index into the `u64` allocation array. As stated, these are `Little` endian.
This is ceil(log<sub>2</sub> `N`) number of bits.

This gives the following representation for the organization of our 832-bit storage array:

```text,ignore
   bit pos     9876 543 210  
   endianness  LLLL LLL LLL  
   field size    13   0   0   (mod power of 2)
```

Now, say we want to iterate over the array as bytes so we can write it out as hex in the
conventional order that we write digits. We need to convert it to:

```text,ignore
   bit pos     9876 543 210  
   endianness  LLLL LLL LLL   storage array format
   endianness  BBBB BBB LLL   desired index format
   field size    13   0   0   (mod power of 2)
```

Taking it one group at a time from LSB to MSB (right-to-left), we see that:

* The first group (`0..=2`, intra-byte bits), `LLL`, does not change.

* The second group (`3..=5`, intra-allocation unit bytes) `LLL`, changes to `BBB`.

* The third group (`6..=9` array elements), `LLLL` changes to `BBBB(13)`.

So to take an index representing `u8` elements in overall big-endian order and
adjust it for accessing storage `&[T = u64]` as if it were `&[M = u8]`:

```rust,ignore
fn adjust_access_index<
    T = u64, const N: usize = 13, // storage type  [T; N]
    U =  u8, const M: usize,      // access type  &[U; M]
>(
    ref_storage_array: &'a [T; N],
    access_index: usize
) -> (
    &'a [U; M], // ref to access array
    usize       // index into access array
) {
    type T = u64; // storage type
    type U = u8;  // access type

    let T_BITS_L2 = 6; // log2(T::BITS)
    let U_BITS_L2 = 3; // log2(A::BITS)
    let T_M_BITS_L2_DIFF = T_BITS_L2 - U_BITS_L2; // 3

    const M: usize = N << T_A_BITS_DIFF_L2; // by how many bits are we increasing the index

    let valid_storage_index_range = 0usize .. N;
    let valid_access_index_range  = 0usize .. M;
    assert!(access_index_range.contains(access_index));

    let add        =       N    << 6; // log2(u64::BITS) - log2(u8::BITS) = 6
    let storage_be = 0b_0000_000_000; // 0 for little, 1 for big
    let access_be  = 0b_1111_111_000;
    let inv_mask   = 0b_1111_111_000; // storage_be^access_be
    let adj        =      13 << T_BITS_L2;

    let access_bit_index = access_index << U_BITS_L2;
    let adjusted_bit_index = (access_bit_index + adj) ^ inv_mask;
    let adjusted_index = adjusted_bit_index >> U_BITS_L2;

    let ref_access_array:  = unsafe { std::mem::transmute::<&'a [T; N], &'a [U; M]>(storage_array) };

    (ref_access_array, adjusted_index)
}
```

#### But wait, there's more

Some types we might really like to use, such as `[std::simd::Simd<u32, 16>]` or
`[core::arch::x86::__m512bh]` are also smaller-things-inside-larger-things, and as such
they introduce another level of indexing and the potential for endianness mismatch. We
don't support them here yet, except to note that these methods are straightforward to
extend to handle additional levels of indexing.
*/

pub mod bitvec_organization;
pub mod endian;
pub mod fixeduint;
pub mod primitive_unsigned;
pub mod with_t_upt;

#[cfg(show_teprintln)]
#[macro_export]
macro_rules! teprintln {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}

#[cfg(not(show_teprintln))]
#[macro_export]
macro_rules! teprintln {
    ($($arg:tt)*) => { };
}
