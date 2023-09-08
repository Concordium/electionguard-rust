#[cfg(test)]
mod t_bitvec_organization {
    use super::*;
    use crate::for_each_fixed_width_unsigned_primitive_type;
    use crate::primitive_unsigned::*;

    /* macro_rules! for_bits_l2_val {
        (let $id:ident: $id_T:ident = $e:expr ; $( $s:stmt );*) => {
            match T::BITS_L2 {
                3 => for_bits_l2_val!(@expand; u8; $id; $id_T; $e; $( $s );*),
                4 => for_bits_l2_val!(@expand; u16; $id; $id_T; $e; $( $s );*),
                _ => { panic!("unexpected PrimitiveType::BITS_L2") },
            }
        };

        (@expand; $pt:ident; $id:ident; $id_T:ident; $e:expr; $( $s:stmt );*) => {{
            type $id_T = $pt;
            assert!(std::mem::size_of::<$id_T>() == std::mem::size_of::<T>());
            assert!($id_T::SIZE == std::mem::size_of::<T>());

            //let $id: $id_T = ($e) as $id_T; // truncation in unused cases
            let $id = ($e) as $id_T; // truncation in unused cases

            //let $id: $id_T = unsafe { std::mem::transmute::<_, $id_T>($id) };

            let $id: $id_T = { $( $s );* };

            * unsafe { std::mem::transmute::<& $id_T, & T>(& $id) }
        }};
    } */

    // Converts output to T
    macro_rules! with_t_upt_output {
        ($T:ident, $CallerUPT:ident => $block:block) => {
            match <T as $crate::primitive_unsigned::PrimitiveType>::BITS_L2 {
                3 => with_t_upt_output!(@expand;  u8; $T; $CallerUPT; $block),
                4 => with_t_upt_output!(@expand; u16; $T; $CallerUPT; $block),
                5 => with_t_upt_output!(@expand; u32; $T; $CallerUPT; $block),
                6 => with_t_upt_output!(@expand; u64; $T; $CallerUPT; $block),
                7 => with_t_upt_output!(@expand; u128; $T; $CallerUPT; $block),
                _ => {
                    // The first parameter to this macro must be the name of a
                    // generic parameter bound to a PrimitiveType type.
                    // I.e., one of `u8`, `u16`, `u32`, `u64`, or `u128`.
                    const fn please_bound_first_macro_parameter_type_as_follows<$T>() -> bool
                        where
                            $T: $crate::primitive_unsigned::PrimitiveType<PrimitiveType = $T>
                    { true }
                    please_bound_first_macro_parameter_type_as_follows::<$T>();
                    unreachable!()
                },
            }
        };

        (@expand; $PT:ident; $T:ident; $CallerUPT:ident; $block:block) => {{
            // This is what makes it safe transmute between the native type and T,
            // which is dynamically the same time.
            assert!(std::mem::size_of::<T>() == std::mem::size_of::<$PT>());
            assert!(std::mem::align_of::<T>() == std::mem::align_of::<$PT>());

            type $CallerUPT = $PT;

            let _with_unsigned_primitive_t_var: $CallerUPT = $block;

            * unsafe { std::mem::transmute::<& $CallerUPT, & T>(& _with_unsigned_primitive_t_var) }
        }};
    }

    // Returns the requested number of 1-valued bits, up to `T::BITS`.
    const fn ones<T>(n: u32) -> T
    where
        T: PrimitiveType<PrimitiveType = T>,
    {
        if T::BITS <= n {
            T::MAX
        } else {
            with_t_upt_output!(T, PrimT => {
                (PrimT::ONE << n) - 1
            })
        }
    }

    #[test]
    fn t1() {
        for_each_fixed_width_unsigned_primitive_type!(T =>
            for n in 0 ..= (T::BITS + 1) {
                let actual: T = ones(n);
                let expected_ones = T::BITS.min(n);
                assert_eq!(actual.count_ones(), expected_ones);
                //eprintln!(" ones::<u{width}>({n}) -> 0b_{actual:0width$b}_u{width}", width = T::BITS as usize);
            }
        );
    }

    /* // Does not convert
    macro_rules! with_t_upt {
        ($T:ident, $CallerUPT:ident => $block:block) => {
            match <T as $crate::primitive_unsigned::PrimitiveType>::BITS_L2 {
                3 => with_t_upt!(@expand;  u8; $T; $CallerUPT; $block),
                4 => with_t_upt!(@expand; u16; $T; $CallerUPT; $block),
                5 => with_t_upt!(@expand; u32; $T; $CallerUPT; $block),
                6 => with_t_upt!(@expand; u64; $T; $CallerUPT; $block),
                7 => with_t_upt!(@expand; u128; $T; $CallerUPT; $block),
                _ => {
                    // The first parameter to this macro must be the name of a
                    // generic parameter bound to a PrimitiveType type.
                    // I.e., one of `u8`, `u16`, `u32`, `u64`, or `u128`.
                    const fn please_bound_first_macro_parameter_type_as_follows<$T>() -> bool
                        where
                            $T: $crate::primitive_unsigned::PrimitiveType<PrimitiveType = $T>,
                    { true }
                    please_bound_first_macro_parameter_type_as_follows::<$T>();
                    unreachable!()
                },
            }
        };

        (@expand; $PT:ident; $T:ident; $CallerUPT:ident; $block:block) => {{
            // This is what makes it safe transmute between the native type and T,
            // which is dynamically the same time.
            assert!(std::mem::size_of::<T>() == std::mem::size_of::<$PT>());
            assert!(std::mem::align_of::<T>() == std::mem::align_of::<$PT>());

            type $CallerUPT = $PT;

            const fn t_to_pt<U>(u: U) -> $PT
            where
                U: $crate::primitive_unsigned::PrimitiveType,
                U: $crate::primitive_unsigned::PrimitiveType<PrimitiveType = U>,
                //U: $crate::primitive_unsigned::PrimitiveType<PrimitiveType = $PT>,
            {
                assert!(std::mem::size_of::<U>() == std::mem::size_of::<$PT>());
                assert!(std::mem::align_of::<U>() == std::mem::align_of::<$PT>());

                let v = u;
                unsafe { std::ptr::read(std::ptr::addr_of!(v) as *const U as *const $PT) }
            }
            /* const fn t_to_pt(n: $CallerUPT) -> $PT {
                unsafe { std::mem::transmute::<$CallerUPT, $PT>(n) }
            }

             */

            // "can't use generic parameters from outer function"
            /* const fn t_to_pt(n: T) -> $PT {
                unsafe { std::mem::transmute::<T, $PT>(n) }
            }
            const fn pt_to_t(n: $PT) -> T {
                unsafe { std::mem::transmute::<$PT, T>(n) }
            } */

            $block
        }};
    }

    /// Returns the log2 of smallest power of 2 not less than `n`.
    /// As a special case (to avoid panic) returns 0 if `n` is 0.
    const fn ceil_log2<T>(n: T) -> u32
    where
        T: PrimitiveType<PrimitiveType = T>,
        //T: PrimitiveType<PrimitiveType = u8>,
    {
        //let refn = &n;
        with_t_upt!(T, PrimT => {
            let n: PrimT = t_to_pt::<T>(n);
            if n == 0 {
                0_u32
            } else {
                let floor_log2 = n.ilog2();
                (n + ones::<PrimT>(floor_log2)).ilog2()
            }
        })
    }

    #[test]
    fn t2() {
        // assert_eq!( ceil_log2(0u8), 0 );
        assert_eq!( ceil_log2(1u8), 0 );
        // assert_eq!( ceil_log2(2u8), 1 );
        // assert_eq!( ceil_log2(3u8), 2 );
        // assert_eq!( ceil_log2(4u8), 2 );
        // assert_eq!( ceil_log2(5u8), 0 );
    }
    */
} // t_bitvec_organization
