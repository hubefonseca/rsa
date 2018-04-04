use num::{BigInt, One, Zero};
use num::bigint::ToBigInt;
use num::Integer;

// adapted from https://github.com/simon-andrews

pub fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();

    if a == &zero {
        return (*b, zero, one);
    } else {
        let (g, x, y) = egcd(&(b % a), a);
        return (g, y - (*b / *a) * x, x);
    }
}

pub fn modinverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let one: BigInt = One::one();
    let (g, x, _) = egcd(a, m);

    if g != one {
        return None;
    } else {
        return Some(x % m);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modinverse() {
        assert_eq!(modinverse(&3.to_bigint().unwrap(), &7.to_bigint().unwrap()).unwrap(), 5.to_bigint().unwrap());
    }
}
