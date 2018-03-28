extern crate rand;

use num::bigint::{RandBigInt, ToBigUint};
use num::{Zero, One, BigUint, ToPrimitive};
use num::Integer;

pub fn is_prime(n: &BigUint) -> bool {
    miller_rabin(n, 5)
}

fn miller_rabin(candidate: &BigUint, limit: usize) -> bool {
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;

    // ( a ^ r ) * d
    let (r, d) = rd_factor(&(candidate - one.clone()));

    let n_minus_one = candidate - one.clone();

    let mut thread_rng = rand::thread_rng();

    for i in 0..10 {
        let a = thread_rng.gen_biguint_range(&two.clone(), &(candidate - &two));

        let mut v = BigUint::modpow(&a, &d, candidate);

        if v == one.clone() || v == n_minus_one {
            continue;
        }

        let times = (r.clone() - one.clone()).to_u16().unwrap();

        let mut k = 1;
        while k < times {
            v = BigUint::modpow(&v, &two.clone(), candidate);

            if v == one.clone() {
                println!("not prime!");
                return false;
            }
            if v == n_minus_one {
                break;
            }
            k = k + 1;
        }

        if k == times {
            return false;
        }
    }

    true
}

fn rd_factor(n: &BigUint) -> (BigUint, BigUint) {
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;

    let mut d = n.clone();
    let mut r: BigUint = zero.clone();

    while d.is_even() {
        d = d / two.clone();
        r = r + one.clone();
    }

    (r, d)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime() {
        assert_eq!(true, is_prime(&523.to_biguint().unwrap()));
        assert_eq!(false, is_prime(&525.to_biguint().unwrap()));

        assert_eq!(false, is_prime(&179426337.to_biguint().unwrap()));
        assert_eq!(true, is_prime(&179426339.to_biguint().unwrap()));

        assert_eq!(true, is_prime(&179426491.to_biguint().unwrap()));
        assert_eq!(false, is_prime(&179426493.to_biguint().unwrap()));

        assert_eq!(false, is_prime(&15487453.to_biguint().unwrap()));
        assert_eq!(true, is_prime(&15487019.to_biguint().unwrap()));
    }
}
