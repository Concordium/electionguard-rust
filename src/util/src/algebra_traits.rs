use core::fmt::Debug;
use rand::RngCore;
use serde::Serialize;
use subtle::Choice;

/// This trait represents an element of a field.
/// The trait essentially copies `ff::Field` from `v0.5`.
pub trait PrimeField: 
    Serialize
    + Sized 
    + Eq 
    + Copy 
    + Clone 
    + Send 
    + Sync 
    + Debug {
    
    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random(rng: impl RngCore) -> Self;

    /// Returns the zero element of the field, the additive identity.
    fn zero() -> Self;

    /// Returns the one element of the field, the multiplicative identity.
    fn one() -> Self;

    /// Returns true iff this element is zero.
    fn is_zero(&self) -> Choice;

    /// Squares this element.
    fn square(&mut self);

    /// Doubles this element.
    fn double(&mut self);

    /// Negates this element.
    fn negate(&mut self);
    
    
    fn add(self, other: &Self) -> Self;
    fn sub(self, other: &Self) -> Self;
    fn mul(self, other: &Self) -> Self;

    /// Adds another element to this element.
    fn add_assign(&mut self, other: &Self);

    /// Subtracts another element from this element.
    fn sub_assign(&mut self, other: &Self);

    /// Multiplies another element by this element.
    fn mul_assign(&mut self, other: &Self);

    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inverse(&self) -> Option<Self>;

    /// Exponentiates this element by a number represented with `u64` limbs,
    /// least significant digit first. This operation is variable time with
    /// respect to `self`, for all exponent.
    fn pow_vartime<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        // Note: this implementations is
        // copied from the `ff` crate's trait method `ff::Field::pow_vartime()`.
        // https://docs.rs/ff/0.13.0/src/ff/lib.rs.html#178-191
        let mut res = Self::one();
        for e in exp.as_ref().iter().rev() {
            for i in (0..64).rev() {
                res.square();

                if ((*e >> i) & 1) == 1 {
                    res.mul_assign(self);
                }
            }
        }

        res
    }
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