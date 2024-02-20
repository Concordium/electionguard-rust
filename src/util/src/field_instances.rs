use crypto_bigint::{Random, Zero, U256};
use serde::Serialize;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use subtle::{Choice, ConstantTimeEq};

use crate::algebra_traits::PrimeField;

pub const STANDARD_MODULUS: U256 =
    U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43");

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct StandardParamField(U256);

impl From<U256> for StandardParamField {
    fn from(value: U256) -> Self {
        StandardParamField(value)
    }
}

impl Add for StandardParamField {
    type Output = StandardParamField;

    fn add(self, rhs: Self) -> Self::Output {
        self.0.add_mod(&rhs.0, &STANDARD_MODULUS).into()
    }
}

impl AddAssign for StandardParamField {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl super::algebra_traits::Zero for StandardParamField {
    fn zero() -> Self {
        U256::ZERO.into()
    }

    fn is_zero(&self) -> Choice {
        self.0.is_zero()
    }
}

impl Sub for StandardParamField {
    type Output = StandardParamField;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign for StandardParamField {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Mul for StandardParamField {
    type Output = StandardParamField;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for StandardParamField {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl super::algebra_traits::One for StandardParamField {
    fn one() -> Self {
        U256::ONE.into()
    }

    fn is_one(&self) -> Choice {
        self.0.ct_eq(&U256::ONE)
    }
}

impl Div for StandardParamField {
    type Output = StandardParamField;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl DivAssign for StandardParamField {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Neg for StandardParamField {
    type Output = StandardParamField;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl PrimeField for StandardParamField {
    fn square(&mut self) {
        todo!()
    }

    fn double(&mut self) {
        todo!()
    }

    fn inverse(&self) -> Option<Self> {
        todo!()
    }

    fn random<R>(rng: &mut R) -> Self
    where
        R: rand::prelude::RngCore + rand::prelude::CryptoRng,
    {
        U256::random(rng).into()
    }
}
