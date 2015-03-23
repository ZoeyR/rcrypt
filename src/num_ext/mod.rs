use num::bigint::{RandBigInt, BigInt};
use num::{Zero, One, pow};
use std::num::{FromPrimitive};
use num::integer::Integer;
use rand::thread_rng;

/// Cryptographically useful extensions to the provided BigInt functionality.
pub trait BigIntCrypto {
    /// Find the next prime from the current BigInt
    fn next_prime(&self) -> BigInt;

    /// use the extended euclidean algorithm to solve for (g,x,y) given (a,b) such that
    /// g = gcd(a,b) = a*x + b*y.
    fn gcdext(&self, other: &BigInt) -> (BigInt, BigInt, BigInt);

    /// Is this number a prime number. Uses a probablistic function to determine primality.
    fn is_prime(n: &BigInt) -> bool;
}

impl BigIntCrypto for BigInt {
    fn next_prime(&self) -> BigInt {
        let one: BigInt = One::one();
        let two = BigInt::from_isize(2).unwrap();
        let mut nextPrime = self.clone();
        if &nextPrime % &two == Zero::zero() {
            nextPrime = &nextPrime + &one;
        } else {
            nextPrime = &nextPrime + &two;
        }
        while !BigInt::is_prime(&nextPrime) {
            nextPrime = &nextPrime + &two;
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
        if *n < two || n % two == Zero::zero() {
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
        d = d >> 1;
        s = s + &One::one();
    }

    for j in 0..k {
        //println!("loop {} of {}", j, k);
        let a = thread_rng().gen_bigint_range(&two, &(n - &two));
        let mut x = mod_exp(&a, &d, n);
        //let mut x = two.clone();
        if (x == one) || (x == (n - &one)) {
            continue;
        }

        // Use a while loop instead of for here because range does not accept BigInt
        let mut i: BigInt = Zero::zero();
        loop  {
            x = mod_exp(&x, &two, n);
            if x == one || i == (&s - &one) {
                return false;
            }
            if x == (n - &one) {
                break;
            }
            i = i + &one;
        }
    }
    true
}

fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    let zero = Zero::zero();
    let one: BigInt = One::one();
    let two = &one + &one;
    let mut result: BigInt = One::one();
    let mut base_acc = base.clone();
    let mut exp_acc = exponent.clone();
    while exp_acc > zero {
        if (&exp_acc % &two) == one {
            result = (result * &base_acc) % modulus;
        }
        exp_acc = exp_acc >> 1;
        base_acc = (&base_acc * &base_acc) % modulus;
    }
    result
}

mod test_bigint_crypto {
    use super::{BigIntCrypto, mod_exp};
    use num::bigint::{RandBigInt, BigInt};
    use std::num::FromPrimitive;
    use num::One;
    use rand::thread_rng;

    #[test]
    fn next_prime_test() {
        let test_num = BigInt::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875258".as_bytes(), 10).unwrap();

        let expected_next = BigInt::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();

        assert!(test_num.next_prime() == expected_next);
    }

    #[test]
    fn mod_exp_test() {
        let base = BigInt::from_isize(4).unwrap();
        let exponent = BigInt::from_isize(13).unwrap();
        let modulus = BigInt::from_isize(497).unwrap();
        let expected_result = BigInt::from_isize(445).unwrap();

        assert!(mod_exp(&base, &exponent, &modulus) == expected_result);

        // time test, this takes a long time to do!
        let large_num = BigInt::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();
        let a = thread_rng().gen_bigint_range(&One::one(), &large_num);
        let b = thread_rng().gen_bigint_range(&One::one(), &large_num);
        let c = thread_rng().gen_bigint_range(&One::one(), &large_num);
        for _ in 0..100 {
            mod_exp(&a, &b, &c);
        }
    }

    #[test]
    fn is_prime_test() {
        let known_prime = BigInt::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();

        assert!(BigInt::is_prime(&known_prime));
    }

}
