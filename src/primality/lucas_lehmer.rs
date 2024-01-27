use num_bigint::BigUint;
use num_traits::{ One, Zero };
use crate::operations::pow;

/// Performs the Lucas-Lehmer test for Mersenne primes.
///
/// The Lucas-Lehmer test is a primality test for Mersenne numbers, which are numbers of the form 2^p - 1,
/// where p is a prime number. This function checks if the Mersenne number corresponding to the given power
/// is prime.
///
/// # Arguments
///
/// * `power` - A reference to a `BigUint` representing the power `p` in the Mersenne number 2^p - 1.
///
/// # Returns
///
/// * `true` if the Mersenne number 2^p - 1 is prime.
/// * `false` if the number is composite or if `power` is less than or equal to 1. Note that for power = 2, the function returns `true` as 2^2 - 1 = 3 is prime.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// let power = BigUint::from(3u32);
/// assert!(!lucas_lehmer_test(&power));
///
/// let prime_power = BigUint::from(7u32);
/// assert!(lucas_lehmer_test(&prime_power));
/// ```
pub fn lucas_lehmer_test(power: &BigUint) -> bool {
    if power <= &BigUint::from(1u32) {
        return false;
    }
    if power == &BigUint::from(2u32) {
        return true;
    }

    let mersenne = pow(&BigUint::from(2u32), power) - BigUint::one();
    let mut sum = BigUint::from(4u32);

    let mut i = BigUint::zero();
    while i < power - &BigUint::from(2u32) {
        sum = (sum.clone() * sum.clone() - BigUint::from(2u32)) % &mersenne;
        i += BigUint::one();
    }

    sum == BigUint::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_test() {
        let powers = vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203];
        for power in powers {
            println!("Testing M{}", power);
            assert!(lucas_lehmer_test(&BigUint::from(power as u32)));
        }
    }

    #[test]
    fn composite_test() {
        let powers = vec![
            11,
            23,
            29,
            37,
            41,
            43,
            47,
            53,
            59,
            67,
            71,
            73,
            79,
            83,
            97,
            101,
            103,
            109,
            113,
            131,
            137,
            139,
            149,
            151,
            157,
            163,
            167,
            173,
            179
        ];
        for power in powers {
            println!("Testing M{}", power);
            assert!(!lucas_lehmer_test(&BigUint::from(power as u32)));
        }
    }
}
