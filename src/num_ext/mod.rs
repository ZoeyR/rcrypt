use num::bigint::{BigInt};
use num::{Zero, One};

/// Cryptographically useful extensions to the provided BigInt functionality.
pub trait BigIntCrypto {
    /// Find the next prime from the current BigInt
    fn nextprime(&self) -> BigInt;

    /// use the extended euclidean algorithm to solve for (g,x,y) given (a,b) such that
    /// g = gcd(a,b) = a*x + b*y.
    fn gcdext(&self, other: &BigInt) -> (BigInt, BigInt, BigInt);
}

impl BigIntCrypto for BigInt {
    fn nextprime(&self) -> BigInt {

        Zero::zero()
    }

    fn gcdext(&self, other: &BigInt) -> (BigInt, BigInt, BigInt) {

        (Zero::zero(), Zero::zero(), Zero::zero())
    }
}
