use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;
use crate::operations::{ pow, pow_mod };
use crate::operations::utils::get_trailing_zeros;

/// Performs the Miller-Rabin primality test.
///
/// The Miller-Rabin test is a probabilistic primality test: it can prove that a number is composite,
/// but it cannot prove that a number is prime. This implementation uses a set of deterministic
/// witnesses for numbers less than 2^64, which are 2, 3, 5, 7, 11.
///
/// # Arguments
///
/// * `num` - A reference to a `BigUint` representing the number to test for primality.
///
/// # Returns
///
/// * `true` if `num` passes the Miller-Rabin primality test for all witnesses.
/// * `false` if `num` fails the test for any witness, or if `num` is less than or equal to 1.
///   Note that the function returns `true` when `num` is 2, as it is the only even prime number.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// let number = BigUint::from(19u32);
/// assert!(miller_rabin(&number));
///
/// let non_prime = BigUint::from(18u32);
/// assert!(!miller_rabin(&non_prime));
/// ```
///
/// # Note
///
/// This implementation assumes the presence of other functions like `get_trailing_zeros` and `pow_mod`
/// for getting the number of trailing zeros in the binary representation of a number and for
/// performing modular exponentiation, respectively. Ensure these functions are correctly implemented.
pub fn miller_rabin(num: &BigUint) -> bool {
    // Miller Rabin test for witnesses 2, 3, 5, 7, 11, 13, 17, 19, 23, 29

    if *num <= BigUint::one() {
        return false;
    }

    if *num == BigUint::from(2u32) {
        return true;
    }

    // Get r and d such that num = 2^r * d + 1
    let one_minus_num: BigUint = num - BigUint::one();
    let s: &BigUint = &get_trailing_zeros(&one_minus_num);
    let d: &BigUint = &(one_minus_num / pow(&BigUint::from(2u32), s));

    let switnesses = vec![2, 3, 5, 7, 11];
    let witnesses: Vec<BigUint> = switnesses
        .iter()
        .map(|x| BigUint::from(*x as u32))
        .collect();

    for a in witnesses {
        if a >= *num {
            continue;
        }

        // First Sub Test
        if pow_mod(&a, &d, num) == BigUint::one() {
            continue;
        }

        // Second Sub Test
        let mut found = false;
        let mut r = BigUint::zero();
        while r < *s {
            let a_power = d * pow(&BigUint::from(2u32), &r);
            if (pow_mod(&a, &a_power, num) + BigUint::one()) % num == BigUint::zero() {
                found = true;
                break;
            }
            r = r + BigUint::one();
        }

        if !found {
            println!("Miller Rabin test failed for {}, witness {}", num, a);
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::get_max_primes;

    #[test]
    fn edge_cases() {
        // Test case 0: False
        assert_eq!(miller_rabin(&BigUint::zero()), false);

        // Test case 1: False
        assert_eq!(miller_rabin(&BigUint::one()), false);

        // Test case 2: True
        assert_eq!(miller_rabin(&BigUint::from(2u32)), true);

        // Test case 3: True
        assert_eq!(miller_rabin(&BigUint::from(3u32)), true);

        // Test case 4: False
        assert_eq!(miller_rabin(&BigUint::from(4u32)), false);
    }

    #[test]
    fn large_primes() {
        let primes = [
            // 10-12 digit primes
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
            assert_eq!(miller_rabin(&prime), true);
        }
    }

    #[test]
    fn continuous_test() {
        let primes = get_max_primes(100000);

        for prime in primes {
            assert_eq!(miller_rabin(&prime), true);
        }
    }

    #[test]
    fn large_composites() {
        let primes = [
            // 10-12 digit primes
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

        for i in 0..primes.len() {
            for j in 0..primes.len() {
                if i == j {
                    continue;
                }
                let composite =
                    BigUint::parse_bytes(primes[i].as_bytes(), 10).unwrap() *
                    BigUint::parse_bytes(primes[j].as_bytes(), 10).unwrap();
                assert_eq!(miller_rabin(&composite), false);
            }
        }
    }

    #[test]
    fn counter_example() {
        let counter_example = BigUint::parse_bytes(b"2152302898747", 10).unwrap();
        assert_eq!(miller_rabin(&counter_example), true);
    }
}
