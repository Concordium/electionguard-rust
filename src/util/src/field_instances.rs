use crypto_bigint::{
    impl_modulus,
    modular::{BernsteinYangInverter, ConstMontyForm, ConstMontyParams},
    Invert, Odd, PrecomputeInverter, Random, Uint, U256,
};
use rand::{CryptoRng, RngCore};
use subtle::{ConstantTimeEq, CtOption};

use crate::algebra_traits::{AdditionalFieldOps, One, PrimeField, Zero};

impl<M: ConstMontyParams<L>, const L: usize> Zero for ConstMontyForm<M, L> {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> subtle::Choice {
        self.ct_eq(&Self::ZERO)
    }
}

impl<M: ConstMontyParams<L>, const L: usize> One for ConstMontyForm<M, L> {
    fn one() -> Self {
        Self::ONE
    }

    fn is_one(&self) -> subtle::Choice {
        self.ct_eq(&Self::ONE)
    }
}

impl<M: ConstMontyParams<L>, const L: usize, const U: usize> AdditionalFieldOps
    for ConstMontyForm<M, L>
where
    Odd<Uint<L>>: PrecomputeInverter<Inverter = BernsteinYangInverter<L, U>, Output = Uint<L>>,
{
    fn random<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng,
    {
        <Self as Random>::random(rng)
    }

    fn inv(&self) -> CtOption<Self> {
        <Self as Invert>::invert(&self)
    }

    fn pow(&self, exponent: &Self) -> Self {
        Self::pow(&self, exponent.as_montgomery())
    }

    fn square(&self) -> Self {
        Self::square(&self)
    }
}

macro_rules! impl_integer_field {
    ($field_name:ident, $modulus_type:ty) => {
        pub type $field_name = ConstMontyForm<$modulus_type, { <$modulus_type>::LIMBS }>;
        impl PrimeField for $field_name {}
    };
}

impl_modulus!(
    StandardModulusQ,
    U256,
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43"
);
impl_integer_field!(StandardField, StandardModulusQ);

#[cfg(test)]
pub mod test_parameter_do_not_use_in_production {
    use crypto_bigint::{
        impl_modulus,
        modular::{ConstMontyForm, ConstMontyParams},
        U64,
    };

    use crate::algebra_traits::PrimeField;

    impl_modulus!(TestQ01, U64, "000000000000007F");
    impl_integer_field!(TestField01, TestQ01);
}
