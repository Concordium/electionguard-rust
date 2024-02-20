use core::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_bigint::BigUint;
use rand::RngCore;
use serde::Serialize;
use subtle::Choice;

// Trait for additive neutral element, similar https://docs.rs/num/latest/num/traits/trait.Zero.html
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`, `0`.
    fn zero() -> Self;

    /// Sets `self` to the additive identity element of `Self`, `0`.
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }

    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> Choice;
}

// Trait for multiplicative neutral element, similar https://docs.rs/num/latest/num/traits/trait.One.html
pub trait One: Sized + Mul<Self, Output = Self> {
    // Required method
    fn one() -> Self;

    /// Sets `self` to the multiplicative identity element of `Self`, `1`.
    fn set_one(&mut self) {
        *self = One::one();
    }

    /// Returns `true` if `self` is equal to the multiplicative identity.
    fn is_one(&self) -> Choice;
}

/// This trait represents an element of a prime field Z_q.
pub trait PrimeField: 
    Serialize
    + Sized 
    + Eq 
    //+ Copy 
    + Clone 
    + Send 
    + Sync 
    + Debug 
    + Zero
    + One
    + Add<Self, Output = Self> 
    + Sub<Self, Output = Self> 
    + AddAssign<Self> 
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Neg<Output = Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    {

    const MODULUS: BigUint;
    const NUM_BITS: u32;
    const CAPACITY: u32;
    
    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random(rng: impl RngCore) -> Self;

    /// Squares this element.
    fn square(&mut self);

    /// Doubles this element.
    fn double(&mut self);
    
    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inverse(&self) -> Option<Self>;

}

/// This trait represents an element of a cryptographic, prime-order group.
pub trait PrimeGroup:
    Serialize
    + Clone
    + Copy
    + Debug
    + Eq
    + Sized
    + Send
    + Sync
    + 'static
{
    /// Scalars modulo the order of this group's scalar field.
    type Scalar: PrimeField;

    /// Returns an element chosen uniformly at random from the non-identity elements of
    /// this group.
    ///
    /// This function is non-deterministic, and samples from the user-provided RNG.
    fn random(rng: impl RngCore) -> Self;

    /// Returns the additive identity, also known as the "neutral element".
    fn identity() -> Self;

    /// Returns a fixed generator of the prime-order subgroup.
    fn generator() -> Self;

    /// Determines if this point is the identity.
    fn is_identity(&self) -> Choice;

    /// Doubles this element.
    #[must_use]
    fn double(&self) -> Self;
}