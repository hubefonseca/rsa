extern crate rand;

use num::bigint::{RandBigInt, ToBigInt};
use num::{Zero, One, BigInt, ToPrimitive};
use num::Integer;

pub fn is_prime(n: &BigInt) -> bool {
    if n.is_even() {
        return false;
    }

    miller_rabin(n, 5)
}

fn miller_rabin(candidate: &BigInt, limit: usize) -> bool {
    let one: BigInt = One::one();
    let two = &one + &one;

    // ( a ^ r ) * d
    let (r, d) = rd_factor(&(candidate - one.clone()));

    let n_minus_one = candidate - one.clone();

    let mut thread_rng = rand::thread_rng();

    for i in 0..10 {
        let a = thread_rng.gen_bigint_range(&two.clone(), &(candidate - &two));

        let mut v = BigInt::modpow(&a, &d, candidate);

        if v == one.clone() || v == n_minus_one {
            continue;
        }

        let times = (r.clone() - one.clone()).to_u16().unwrap();

        let mut k = 1;
        while k < times {
            v = BigInt::modpow(&v, &two.clone(), candidate);

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

fn rd_factor(n: &BigInt) -> (BigInt, BigInt) {
    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    let two = &one + &one;

    let mut d = n.clone();
    let mut r: BigInt = zero.clone();

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
        assert_eq!(true, is_prime(&523.to_bigint().unwrap()));
        assert_eq!(false, is_prime(&525.to_bigint().unwrap()));

        assert_eq!(false, is_prime(&179426337.to_bigint().unwrap()));
        assert_eq!(true, is_prime(&179426339.to_bigint().unwrap()));

        assert_eq!(true, is_prime(&179426491.to_bigint().unwrap()));
        assert_eq!(false, is_prime(&179426493.to_bigint().unwrap()));

        assert_eq!(false, is_prime(&15487453.to_bigint().unwrap()));
        assert_eq!(true, is_prime(&15487019.to_bigint().unwrap()));
    }
}
