use num::bigint::{ToBigUint, RandBigInt, BigUint};
use num::{Zero, One};
use num::integer::Integer;
use rand::thread_rng;
use std::sync::{Arc, mpsc};
use std::thread;

/// Cryptographically useful extensions to the provided BigUint functionality.
pub trait BigUintCrypto {
    /// Find the next prime from the current BigUint
    fn next_prime(&self) -> BigUint;

    fn next_prime_threaded(&self) -> BigUint;
    /// use the extended euclidean algorithm to solve for (g,x,y) given (a,b) such that
    /// g = gcd(a,b) = a*x + b*y.
    fn gcdext(&self, other: &BigUint) -> (BigUint, BigUint, BigUint);

    /// Is this number a prime number. Uses a probablistic function to determine primality.
    fn is_prime(n: &BigUint) -> bool;

    /// perform the function (base^exponent) % modulus using exponentiation by sqauring
    fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint;
}

impl BigUintCrypto for BigUint {
    fn next_prime(&self) -> BigUint {
        next_prime_helper(&self.clone(), false)
    }

    fn next_prime_threaded(&self) -> BigUint {
        next_prime_helper(&self.clone(), true)
    }

    fn gcdext(&self, other: &BigUint) -> (BigUint, BigUint, BigUint) {

        (Zero::zero(), Zero::zero(), Zero::zero())
    }

    fn is_prime(n: &BigUint) -> bool {
        is_prime_helper(n, false)
    }

    fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        let zero = Zero::zero();
        let one: BigUint = One::one();
        let two = &one + &one;
        let mut result: BigUint = One::one();
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
}

fn next_prime_helper(n: &BigUint, thread: bool) -> BigUint {
    let one: BigUint = One::one();
    let two = 2.to_biguint().unwrap();
    let mut next_prime = n.clone();
    if &next_prime % &two == Zero::zero() {
        next_prime = &next_prime + &one;
    } else {
        next_prime = &next_prime + &two;
    }
    while !is_prime_helper(&next_prime, thread) {
        next_prime = &next_prime + &two;
    }
    next_prime
}

fn is_prime_helper(n: &BigUint, thread: bool) -> bool {
    let two = 2.to_biguint().unwrap();
    let three = 3.to_biguint().unwrap();
    if *n == three || *n == two {
        return true;
    }
    if *n < two || n % two == Zero::zero() {
        return false;
    }
    miller_rabin(n, 100, thread)
}
/// n must be greater than 3 and k indicates the number of rounds
fn miller_rabin(n: &BigUint, k: usize, thread: bool) -> bool{
    let one: BigUint = One::one();
    let (tx, rx) = mpsc::channel();

    let mut d: BigUint = n - &One::one();
    let mut s: BigUint = Zero::zero();
    while d.is_even() {
        d = d >> 1;
        s = s + &one;
    }
    if thread {
        let shared_n = Arc::new(n.clone());
        let shared_d = Arc::new(d);
        let shared_s = Arc::new(s);

        // miller rabin lends itself to being concurrent since a is completely random
        // here we spawn multiple threads to help speed up the process
        for _ in 0..8 {
            let tx = tx.clone();
            //let thread_n = n.clone();
            let shared_d = shared_d.clone();
            let shared_s = shared_s.clone();
            let shared_n = shared_n.clone();
            thread::spawn(move || {
                let result = miller_rabin_thread(&shared_n, &shared_d, &shared_s, k/8);
                tx.send(result);
                });
        }

        for _ in 0..8 {
            if !rx.recv().ok().expect("A thread failed") {
                return false;
            }
        }
    } else {
        return miller_rabin_thread(n, &d, &s, k);
    }
    true
}

fn miller_rabin_thread(n: &BigUint, d: &BigUint, s: &BigUint, k: usize) -> bool {
    let one: BigUint = One::one();
    let two: BigUint = &one + &one;

    for _ in 0..k {
        //println!("loop {} of {}", j, k);
        let a = thread_rng().gen_biguint_range(&two, &(n - &two));
        let mut x = mod_exp(&a, d, n);
        //let mut x = two.clone();
        if (x == one) || (x == (n - &one)) {
            continue;
        }

        // Use a while loop instead of for here because range does not accept BigUint
        let mut i: BigUint = Zero::zero();
        loop  {
            x = mod_exp(&x, &two, n);
            if x == one || i == (s - &one) {
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

fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let zero = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;
    let mut result: BigUint = One::one();
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

#[cfg(test)]
mod test_BigUint_crypto {
    use super::{BigUintCrypto, mod_exp};
    use num::bigint::{RandBigInt, BigUint};
    use std::num::FromPrimitive;
    use num::One;
    use rand::thread_rng;

    #[test]
    fn next_prime_test() {
        let test_num = BigUint::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875258".as_bytes(), 10).unwrap();

        let expected_next = BigUint::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();

        assert!(test_num.next_prime() == expected_next);
    }

    #[test]
    fn next_prime_threaded_test() {
        let test_num = BigUint::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875258".as_bytes(), 10).unwrap();

        let expected_next = BigUint::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();

        assert!(test_num.next_prime_threaded() == expected_next);
    }

    #[test]
    fn mod_exp_test() {
        let base = BigUint::from_isize(4).unwrap();
        let exponent = BigUint::from_isize(13).unwrap();
        let modulus = BigUint::from_isize(497).unwrap();
        let expected_result = BigUint::from_isize(445).unwrap();

        assert!(mod_exp(&base, &exponent, &modulus) == expected_result);
    }

    #[test]
    fn is_prime_test() {
        let known_prime = BigUint::
        parse_bytes("4829837983753984028472098472089547098728675098723407520875297".as_bytes(), 10).unwrap();

        assert!(BigUint::is_prime(&known_prime));
    }

    #[test]
    #[should_fail]
    fn is_prime_test_failuire() {
        let not_prime = BigUint::
        parse_bytes("359709793871987301975987296195681798740165298740176567105918720469720137416098423"
        .as_bytes(), 10).unwrap();

        assert!(BigUint::is_prime(&not_prime));
    }

}
