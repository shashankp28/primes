use num_bigint::BigUint;
use num_traits::One;
use crate::operations::pow_mod;
use crate::operations::gcd;

/// Performs a probabilistic primality test using Fermat's little theorem.
///
/// Fermat's little theorem states that if `p` is a prime number, then for any
/// integer `a` such that `1 < a < p - 1`, the number `a^(p-1)` is congruent to 1 modulo `p`.
/// This function tests the primality of `num` using a set of base witnesses (2, 3, 5, 7, 11, 13, 17, 19, 23, 29).
///
/// Note that passing this test does not guarantee that `num` is prime, but failing it
/// guarantees that `num` is composite. Thus, this function can produce false positives,
/// but not false negatives.
///
/// # Arguments
///
/// * `num` - A reference to a `BigUint` representing the number to test for primality.
///
/// # Returns
///
/// * `true` if `num` passes the Fermat primality test for all witnesses.
/// * `false` if `num` fails the test for any witness, or if `num` is less than or equal to 1.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use large_primes::fermat;
/// 
/// let number = BigUint::parse_bytes(b"97", 10).unwrap();
/// assert!(fermat(&number));
///
/// let non_prime = BigUint::parse_bytes(b"100", 10).unwrap();
/// assert!(!fermat(&non_prime));
/// ```

pub fn fermat(num: &BigUint) -> bool {
    // Fermat's little theorem test for witnesses 2, 3, 5, 7, 11, 13, 17, 19, 23, 29

    if *num <= BigUint::one() {
        return false;
    }

    let switnesses = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let witnesses: Vec<BigUint> = switnesses
        .iter()
        .map(|x| BigUint::from(*x as u32))
        .collect();
    for witness in witnesses {
        if gcd(&witness, num) != BigUint::one() {
            continue;
        }
        let mod_value = pow_mod(&witness, &num, num);
        let rhs = pow_mod(&witness, &BigUint::one(), &num);
        if mod_value != rhs {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::*;

    #[test]
    fn edge_cases() {
        // Test case 0: False
        assert_eq!(fermat(&BigUint::zero()), false);

        // Test case 1: False
        assert_eq!(fermat(&BigUint::one()), false);

        // Test case 2: True
        assert_eq!(fermat(&BigUint::from(2u32)), true);

        // Test case 3: True
        assert_eq!(fermat(&BigUint::from(3u32)), true);

        // Test case 4: False
        assert_eq!(fermat(&BigUint::from(4u32)), false);
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
            assert_eq!(fermat(&prime), true);
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
                assert_eq!(fermat(&composite), false);
            }
        }
    }

    #[test]
    fn carmichael_number() {
        let carmichaels: Vec<BigUint> = vec![
            BigUint::parse_bytes(b"561", 10).unwrap(),
            BigUint::parse_bytes(b"41041", 10).unwrap(),
            BigUint::parse_bytes(b"825265", 10).unwrap(),
            BigUint::parse_bytes(b"321197185", 10).unwrap(),
            BigUint::parse_bytes(b"5394826801", 10).unwrap(),
            BigUint::parse_bytes(b"232250619601", 10).unwrap(),
            BigUint::parse_bytes(b"9746347772161", 10).unwrap(),
            BigUint::parse_bytes(b"1436697831295441", 10).unwrap(),
            BigUint::parse_bytes(b"60977817398996785", 10).unwrap(),
            BigUint::parse_bytes(b"7156857700403137441", 10).unwrap()
        ];

        for carmichael in carmichaels {
            assert_eq!(fermat(&carmichael), true);
        }
    }
}
