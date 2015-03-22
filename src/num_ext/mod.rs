use num::bigint::{RandBigInt, BigInt};
use num::{Zero, One};
use std::num::FromPrimitive;
use num::integer::Integer;
use rand::thread_rng;

/// Cryptographically useful extensions to the provided BigInt functionality.
pub trait BigIntCrypto {
    /// Find the next prime from the current BigInt
    fn nextprime(&self) -> BigInt;

    /// use the extended euclidean algorithm to solve for (g,x,y) given (a,b) such that
    /// g = gcd(a,b) = a*x + b*y.
    fn gcdext(&self, other: &BigInt) -> (BigInt, BigInt, BigInt);

    /// Is this number a prime number. Uses a probablistic function to determine primality.
    fn is_prime(n: &BigInt) -> bool;
}

impl BigIntCrypto for BigInt {
    fn nextprime(&self) -> BigInt {
        let two = BigInt::from_isize(2).unwrap();
        let mut nextPrime = self.clone();

        while !BigInt::is_prime(&nextPrime) {
            nextPrime = nextPrime + &two;
        }
        nextPrime
    }

    fn gcdext(&self, other: &BigInt) -> (BigInt, BigInt, BigInt) {

        (Zero::zero(), Zero::zero(), Zero::zero())
    }

    fn is_prime(n: &BigInt) -> bool {
        let two = BigInt::from_isize(2).unwrap();
        let three = BigInt::from_isize(3).unwrap();
        if *n == three || *n == two {
            return true;
        }
        if *n < two|| n % two == Zero::zero() {
            return false;
        }
        miller_rabin(n, 100)
    }


}

/// n must be greater than 3 and k indicates the number of rounds
fn miller_rabin(n: &BigInt, k: usize) -> bool{
    let one: BigInt = One::one();
    let two: BigInt = &one + &one;
    let mut d: BigInt = n - &One::one();
    let mut s: BigInt = Zero::zero();

    while d.is_even() {
        d = d >> 2;
        s = s + &One::one();
    }

    for _ in 0..k {
        let a = thread_rng().gen_bigint_range(&two, &(n - &two));
    }
    false
}
