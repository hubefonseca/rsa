extern crate rand;

use num::bigint::RandBigInt;
use num::{Zero, One, BigUint};
use num::Integer;

pub fn is_prime(n: &BigUint) -> bool {
    miller_rabin(n, 5)
}

fn miller_rabin(candidate: &BigUint, limit: usize) -> bool {
    let one = One::one();
    let two = &one + &one;
    let (s, d) = rewrite(&(candidate - one));

    let mut thread_rng = rand::thread_rng();

    for _ in 0..limit {
        let basis = thread_rng.gen_biguint_range(&two, &(candidate-&two));
        let mut y = BigUint::modpow(&basis, &d, candidate);

        if y == one || y == (candidate - &one) {
            continue;
        } else {
            let mut k: BigUint = One::one();
            while k < s-one.clone() {
//            for _ in one.clone()..s-one.clone() {
                y = BigUint::modpow(&y, &two, candidate);
                if y == one {
                    return false
                } else if y == candidate - &one {
                    break;
                }
                k = k+one.clone();
            }
            return false;
        }
    }
    true
}

fn rewrite(n: &BigUint) -> (BigUint, BigUint) {
    let mut d = n.clone();
    let mut s: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = one + one;

    while d.is_even() {
        d = d / two;
        s = s + one;
    }
    (s, d)
}
