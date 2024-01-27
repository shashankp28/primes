use num_bigint::BigUint;
use num_traits::{ One, Zero };

/// Performs the standard primality test by checking for prime factors from 2 to the square root of the given number.
///
/// This function checks whether the provided number is prime by testing for prime factors in the range from 2
/// to the square root of the number (inclusive). It returns `true` if the number has no prime factors in that range,
/// indicating that it is prime.
///
/// # Arguments
///
/// * `num` - A reference to a `BigUint` representing the number to test for primality.
///
/// # Returns
///
/// * `true` if `num` passes the standard primality test (has no prime factors from 2 to sqrt(num)).
/// * `false` if `num` has a prime factor in the specified range or if `num` is less than or equal to 1.
///   Note that the function returns `true` when `num` is 2, as it is the only even prime number.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use large_primes::standard;
/// 
/// let prime_number = BigUint::from(19u32);
/// assert!(standard(&prime_number));
///
/// let non_prime = BigUint::from(15u32);
/// assert!(!standard(&non_prime));
/// ```
pub fn standard(num: &BigUint) -> bool {
    // Standard test for prime factors from 2 to sqrt(num)+1
    if *num <= BigUint::one() {
        return false;
    }
    if *num == BigUint::from(2u32) {
        return true;
    }

    let sqrt_num = num.sqrt() + BigUint::one();

    let mut factor = BigUint::from(2u32);
    while &factor <= &sqrt_num {
        if num % &factor == BigUint::zero() {
            return false;
        }
        factor += BigUint::one();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_cases() {
        // Test case 0: False
        assert_eq!(standard(&BigUint::zero()), false);

        // Test case 1: False
        assert_eq!(standard(&BigUint::one()), false);

        // Test case 2: True
        assert_eq!(standard(&BigUint::from(2u32)), true);

        // Test case 3: True
        assert_eq!(standard(&BigUint::from(3u32)), true);

        // Test case 4: False
        assert_eq!(standard(&BigUint::from(4u32)), false);
    }

    #[test]
    fn large_primes() {
        let primes = [
            "871603259",
            "98762051",
            "1000000007",
            "123575321",
            "193818613",
            "444444443",
            "999999937",
            "1000000000039",
            "9999999929",
        ];

        for prime in primes {
            let prime = BigUint::parse_bytes(prime.as_bytes(), 10).unwrap();
            assert_eq!(standard(&prime), true);
        }
    }

    #[test]
    fn large_composites() {
        let primes = ["914491", "15959", "767857", "14293", "680123", "617237"];

        for i in 0..primes.len() {
            for j in 0..primes.len() {
                if i == j {
                    continue;
                }
                let composite =
                    BigUint::parse_bytes(primes[i].as_bytes(), 10).unwrap() *
                    BigUint::parse_bytes(primes[j].as_bytes(), 10).unwrap();
                assert_eq!(standard(&composite), false);
            }
        }
    }
}
