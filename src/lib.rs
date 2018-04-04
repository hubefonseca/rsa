extern crate num;
extern crate rand;

use num::bigint::{RandBigInt, ToBigInt};
use num::{BigInt, CheckedSub};
use num::integer::{lcm,gcd};
use num::{One};

use primes::is_prime;
use numbers::modinverse;

pub mod primes;
pub mod numbers;

pub struct PublicKey {
    e: BigInt,
    n: BigInt
}

pub struct PrivateKey {
    d: BigInt,
    n: BigInt
}

pub fn generate(size: usize) -> (BigInt, BigInt, BigInt) {
    let p = random_prime(size / 2);
    let q = random_prime(size / 2);
    let e = 65537.to_bigint().unwrap();

    let one: BigInt = One::one();
    let lambda = lcm(p.checked_sub(&one).unwrap(), q.checked_sub(&one).unwrap());

    let n = &p * &q;
    println!("n {}, lambda {}, -1 {}", n, lambda, &-1.to_bigint().unwrap());

//    let d = BigInt::modpow(&e, &-1.to_bigint().unwrap(), &lambda).to_bigint().unwrap();
//    let d = modinverse(&e, &lambda).unwrap();
    let d = -1.to_bigint().unwrap();

    println!("p {}, q {}, e {}, n {}, d {}", p, q, e, n, d);

    (e, n, d)
}

fn random_prime(bits: usize) -> BigInt {
    let mut thread_rng = rand::thread_rng();

    let mut a: BigInt = 6074001000u64.to_bigint().unwrap();
    let mut b: BigInt = One::one();

    a = a << (bits - 33);
    b = b << bits;

    let one: BigInt = One::one();
    b = b.checked_sub(&one).unwrap();

    let mut p: BigInt = One::one();

    loop {
        p = thread_rng.gen_bigint_range(&a, &b);

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
        assert_eq!((0.to_bigint().unwrap(), 0.to_bigint().unwrap(), 0.to_bigint().unwrap()), generate(8));
    }
}
