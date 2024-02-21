use core::fmt::Debug;
use rand::{CryptoRng, RngCore};
use serde::Serialize;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use subtle::{Choice, CtOption};

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

// Helper trait for all remaining  field operations we need
pub trait AdditionalFieldOps: Sized {
    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng;

    /// Returns the square of this element.
    fn square(&self) -> Self;

    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inv(&self) -> CtOption<Self>;

    /// Raises the element to the `exponent` power.
    fn pow(&self, exponent: &Self) -> Self;
}

/// This trait represents an element of a prime field Z_q.
pub trait PrimeField:
    Serialize
    + Sized
    + Eq
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
    + AdditionalFieldOps
{
}

/// This trait represents an element of a cryptographic, prime-order group 
/// 
/// The trait uses multiplicative notation (as used in the specification). 
pub trait PrimeGroup:
    Serialize + Clone + Debug + Eq + Sized + Send + Sync + 'static
{
    /// Scalars modulo the order of this group's scalar field.
    type Scalar: PrimeField;

    /// A fixed generator of the group
    const G: Self;

    /// Returns an element chosen uniformly at random from the non-identity elements of
    /// this group.
    ///
    /// This function is non-deterministic, and samples from the user-provided RNG.
    fn random<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng;

    /// Returns the multiplicative identity, also known as the "neutral element".
    fn identity() -> Self;

    /// Determines if this point is the identity.
    fn is_identity(&self) -> Choice;

    /// Group multiplication
    #[must_use]
    fn mul(self, rhs: &Self) -> Self;

    /// Group multiplication that mutates the original value
    fn mul_assign(&mut self, rhs: &Self);

    /// Exponentiation with a scalar
    #[must_use]
    fn exp(self, s: Self::Scalar) -> Self;

    /// Exponentiation with a scalar that mutates the original value
    fn exp_assign(&mut self, s: Self::Scalar);

    /// Group inverse
    #[must_use]
    fn inv(&self) -> Self;
}
