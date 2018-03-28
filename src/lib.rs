extern crate num;
extern crate rand;

use num::bigint::{RandBigInt, ToBigUint};
use num::{BigUint, CheckedSub};
use num::integer::lcm;
use num::{One};

use primes::is_prime;

pub mod primes;

pub fn generate(size: usize) -> (BigUint, BigUint, BigUint) {
    let p = random_prime(size);
    let q = random_prime(size);

    println!("p {}, q {}", p, q);

    let one: BigUint = One::one();

    let lambda = lcm(p.checked_sub(&one).unwrap(), q.checked_sub(&one).unwrap());

    (p, q, lambda)
}

fn random_prime(bits: usize) -> BigUint {
    let mut thread_rng = rand::thread_rng();

    let mut a: BigUint = 6074001000u64.to_biguint().unwrap();
    let mut b: BigUint = One::one();

    a = a << (bits - 33);
    b = b << bits;

    let one: BigUint = One::one();
    b = b.checked_sub(&one).unwrap();

    let mut p: BigUint = One::one();

    loop {
        p = thread_rng.gen_biguint_range(&a, &b);

        println!("p {}", p);

        if is_prime(&p) {
            break;
        }

        break;
    }

    println!("random prime: {}", p);

    p
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_generate() {
        assert_eq!((0.to_biguint().unwrap(), 0.to_biguint().unwrap(), 0.to_biguint().unwrap()), generate(8));
    }
}
